import { WebSocketServer, WebSocket } from 'ws';
const wss = new WebSocketServer({ port: 8080 });
// Global variables for a simple 1-to-1 connection.
// NOTE: This limits your app to only TWO concurrent users total. 
// For a production app, use a Map or an Array to handle "Rooms" or "Sessions".
let senderSocket = null;
let receiverSocket = null;
wss.on('connection', function connection(ws) {
    // Good practice: catch socket-level errors
    ws.on('error', console.error);
    ws.on('message', function message(data) {
        let message;
        try {
            // Safely parse JSON. Convert buffer to string first (required in newer ws versions)
            message = JSON.parse(data.toString());
        }
        catch (error) {
            console.error("Invalid JSON received from client");
            return; // Exit early so the server doesn't crash
        }
        console.log("Received message type:", message.type);
        // 1. Register Sender
        if (message.type === 'sender') {
            senderSocket = ws;
            console.log("Sender connected");
        }
        // 2. Register Receiver (Fixed spelling: 'receiver')
        else if (message.type === 'receiver') {
            receiverSocket = ws;
            console.log("Receiver connected");
        }
        // 3. Forward Offer
        else if (message.type === 'createOffer') {
            if (ws !== senderSocket)
                return;
            receiverSocket?.send(JSON.stringify({
                type: 'createOffer',
                sdp: message.sdp
            }));
        }
        // 4. Forward Answer
        else if (message.type === 'createAnswer') {
            if (ws !== receiverSocket)
                return;
            // Fixed bug: Ensure we send 'createAnswer' so the React Sender recognizes it
            senderSocket?.send(JSON.stringify({
                type: 'createAnswer',
                sdp: message.sdp
            }));
        }
        // 5. Forward ICE Candidates
        else if (message.type === 'iceCandidate') {
            if (ws === senderSocket) {
                receiverSocket?.send(JSON.stringify({
                    type: 'iceCandidate',
                    candidate: message.candidate
                }));
            }
            else if (ws === receiverSocket) {
                senderSocket?.send(JSON.stringify({
                    type: 'iceCandidate',
                    candidate: message.candidate
                }));
            }
        }
    });
    // Handle Disconnections
    ws.on('close', () => {
        if (ws === senderSocket) {
            console.log("Sender disconnected");
            senderSocket = null;
        }
        else if (ws === receiverSocket) {
            console.log("Receiver disconnected");
            receiverSocket = null;
        }
    });
});
console.log("WebRTC Signaling Server running on ws://localhost:8080");
//# sourceMappingURL=index.js.map