import { useEffect } from "react";

export const Receiver = () => {
    useEffect(() => {
        //starting a websoket connection
        const socket = new WebSocket('ws://localhost:8080')
        socket.onopen = () => {
            socket.send(JSON.stringify({
                type: 'receiver'
            }))
        }
        // recieving from the function from websocket from the other side
        startReceiving(socket);
    }, [])


    function startReceiving(socket: WebSocket) {
        //rendering the video in DOM
        const video = document.createElement('video');
        document.body.appendChild(video);
        //starting a new RTC peer connection on the recieveing end
        const pc = new RTCPeerConnection();
        // on track receives the video stream from other browser
        pc.ontrack = (event) => {
            console.log(event);
            // to run the video on the DOM 
            video.srcObject = new MediaStream([event.track])
            video.play();
        }

        // to recieve the offer sent by the browser
        socket.onmessage = (event) => {
            //parsing the data form message sent via websocket as event
            const message = JSON.parse(event.data);
            // checking if the message type is createOffer
            if (message.type === 'createOffer') {
                //if yes then set the remote description offer's sdp file 
                pc.setRemoteDescription(message.sdp).then(() => {
                    // then generate your own answer for browser 
                    pc.createAnswer().then((answer) => {
                        // the answer as the local description
                        pc.setLocalDescription(answer);
                        // send the answer to the other browser
                        socket.send(JSON.stringify({
                            type: 'createAnswer',
                            sdp: answer
                        }))
                    })
                })
            } else if (message.type = 'iceCandidate') {
                // adding the ice candidate to the sdp file in local browser
                pc.addIceCandidate(message.candidate)
            }
        }


    }
    return <div></div>

}

