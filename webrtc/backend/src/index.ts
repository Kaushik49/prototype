import { WebSocketServer, WebSocket } from 'ws';

const wss = new WebSocketServer({ port: 8080 })
//caching the ws variable when somebody sends me a message 
// this the variable where you store the ws , when one party sends message
let senderSocket: null | WebSocket = null;
let recieverSocket: null | WebSocket = null;


// createOffer
///create answer
// add ice candidate


wss.on('connection', function connection(ws) {
    ws.on('message', function message(data: any) {
        const message = JSON.parse(data);
        // checking if the request sent by client is sender or reciever
        if (message.type === 'identify-as-sender') {
            // it storest the specific pipe in senderSocker a global variable form ws object
            senderSocket = ws;
        }
        else if (message.type === 'identify-as-reciever') {
            recieverSocket = ws;
        }
        // to check the message type and send the other browser
        else if (message.type === 'create-Offer') {
            recieverSocket?.send(JSON.stringify({
                type: 'offer',
                offer: message.offer

            }))
        }
        // if its answer than you forward it to the originl answer
        else if (message.type === 'create-answer') {
            senderSocket?.send(JSON.stringify({
                type: 'answer',
                offer: message.answer

            }))
        }
        console.log(message);
    });
});
