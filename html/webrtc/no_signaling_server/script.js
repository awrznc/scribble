export class WebRTC {

  constructor() {
    this.config = { 'iceServers': [ { "urls": "stun:stun.l.google.com:19302" } ] };
    this.option = { ordered: false };
    this.channel;
    this.connection;
  }

  setChannel(target) {
    this.channel = target;
    this.channel.onmessage = (event) => console.log('message: ', event.data);
  }

  createConnection() {
    let self = this;
    this.connection = new RTCPeerConnection(this.config);
    this.connection.onicecandidate = (event) => { event.candidate ? console.log('candidate: ', event.candidate) : console.log(self.connection.localDescription.sdp); };
    this.connection.onconnectionstatechange = (event) => console.log('state: ', self.connection.connectionState);
    this.connection.ondatachannel = (event) => self.setChannel( event.channel );
  }

  startPeerConnection() {
    let self = this;
    this.createConnection();
    this.setChannel( this.connection.createDataChannel('example-channel', this.option) );
    this.connection.createOffer().then( (sessionDescription) => self.connection.setLocalDescription(sessionDescription) );
  }

  setRemoteSdp(sdpText) {
    if (this.connection) {
      let answer = new RTCSessionDescription({ type: 'answer', sdp: sdpText });
      this.connection.setRemoteDescription(answer);
    } else {
      let offer = new RTCSessionDescription({ type: 'offer', sdp: sdpText });
      this.createConnection();
      this.connection.setRemoteDescription(offer);
      this.connection.createAnswer().then( (sessionDescription) => this.connection.setLocalDescription(sessionDescription) );
    }
  }

  sendData(data) {
    this.channel.send(data);
  }
}
