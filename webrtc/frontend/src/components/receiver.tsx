import { useEffect, useRef } from "react";

export const Receiver = () => {
    const videoRef = useRef<HTMLVideoElement>(null);

    useEffect(() => {
        const socket = new WebSocket('ws://localhost:8080');
        const pc = new RTCPeerConnection();

        socket.onopen = () => {
            socket.send(JSON.stringify({ type: 'receiver' }));
        };

        pc.ontrack = (event) => {
            console.log("Track received:", event);
            if (videoRef.current) {
                if (event.streams && event.streams[0]) {
                    videoRef.current.srcObject = event.streams[0];
                } else {
                    if (!videoRef.current.srcObject) {
                        videoRef.current.srcObject = new MediaStream([event.track]);
                    } else {
                        (videoRef.current.srcObject as MediaStream).addTrack(event.track);
                    }
                }
            }
        };

        pc.onicecandidate = (event) => {
            if (event.candidate) {
                socket.send(JSON.stringify({
                    type: 'iceCandidate',
                    candidate: event.candidate
                }));
            }
        };

        socket.onmessage = async (event) => {
            const message = JSON.parse(event.data);
            try {
                if (message.type === 'createOffer') {
                    await pc.setRemoteDescription(message.sdp);
                    const answer = await pc.createAnswer();
                    await pc.setLocalDescription(answer);
                    socket.send(JSON.stringify({
                        type: 'createAnswer',
                        sdp: answer
                    }));
                } else if (message.type === 'iceCandidate') {
                    await pc.addIceCandidate(message.candidate);
                }
            } catch (error) {
                console.error("Error during WebRTC negotiation:", error);
            }
        };

        return () => {
            pc.close();
            socket.close();
        };
    })
}