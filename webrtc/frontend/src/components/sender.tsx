import { useEffect, useState, useRef } from "react";

export const Sender = () => {
    const [socket, setSocket] = useState<WebSocket | null>(null);
    const [pc, setPC] = useState<RTCPeerConnection | null>(null);

    // 1. Use a ref to attach the media stream to the video element
    const videoRef = useRef<HTMLVideoElement>(null);

    useEffect(() => {
        const ws = new WebSocket('ws://localhost:8080');
        setSocket(ws);

        ws.onopen = () => {
            ws.send(JSON.stringify({
                type: 'sender'
            }));
        };

        // Cleanup function to close the socket when the component unmounts
        return () => {
            ws.close();
        };
    }, []);

    const initiateConn = async () => {
        if (!socket) {
            alert("Socket not found");
            return;
        }

        // 2. Initialize the PeerConnection BEFORE using it in event listeners
        const peerConnection = new RTCPeerConnection({
            // iceServers:[{ urls: 'stun:stun.l.google.com:19302' }] // Add STUN servers for a real-world app
        });
        setPC(peerConnection);

        socket.onmessage = async (event) => {
            const message = JSON.parse(event.data);
            if (message.type === 'createAnswer') {
                await peerConnection.setRemoteDescription(message.sdp);
            } else if (message.type === 'iceCandidate') {
                await peerConnection.addIceCandidate(message.candidate);
            }
        };

        peerConnection.onicecandidate = (event) => {
            if (event.candidate) {
                socket.send(JSON.stringify({
                    type: 'iceCandidate',
                    candidate: event.candidate
                }));
            }
        };

        peerConnection.onnegotiationneeded = async () => {
            console.log("onnegotiationneeded");
            const offer = await peerConnection.createOffer();
            await peerConnection.setLocalDescription(offer);
            socket.send(JSON.stringify({
                type: 'createOffer',
                sdp: peerConnection.localDescription
            }));
        };

        await getCameraStreamAndSend(peerConnection);
    };

    const getCameraStreamAndSend = async (peerConnection: RTCPeerConnection) => {
        try {
            const stream = await navigator.mediaDevices.getUserMedia({ video: true });

            // Assign the stream to the React video component
            if (videoRef.current) {
                videoRef.current.srcObject = stream;
            }

            stream.getTracks().forEach((track) => {
                console.log("track added", track);
                // 3. Pass the stream as the second argument so the receiver can synchronize tracks
                peerConnection.addTrack(track, stream);
            });
        } catch (error) {
            console.error("Error accessing media devices:", error);
        }
    };

    return (
        <div>
            <h2>Sender</h2>
            {/* Disable the button once the connection has been initiated to prevent duplicates */}
            <button onClick={initiateConn} disabled={!!pc}>
                Send data
            </button>

            <div style={{ marginTop: "20px" }}>
                {/* 
                  autoPlay, playsInline, and muted are critical for 
                  browsers to allow video playback without user interaction 
                */}
                <video
                    ref={videoRef}
                    autoPlay
                    playsInline
                    muted
                    style={{ width: '400px', backgroundColor: '#222', borderRadius: '8px' }}
                />
            </div>
        </div>
    );
};