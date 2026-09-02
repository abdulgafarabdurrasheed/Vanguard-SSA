import { Canvas } from '@react-three/fiber'
import { OrbitControls } from '@react-three/drei'
import './index.css'
import React, { useState, useEffect, Suspense } from 'react'
import { ISS } from './3dmodels';
import { Earth } from './3dmodels';

function App() {
  const [coordinates, setCoordinates] = useState<[number, number, number]>([0, 0, 0]);

  useEffect(() => {
    const socket = new WebSocket("ws://127.0.0.1:3000/ws");
    socket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      console.log("Incoming from Rust:", data);
      setCoordinates([
        data.predicted_xyz[0] / 6371, 
        data.predicted_xyz[2] / 6371, 
        data.predicted_xyz[1] / 6371
      ]);
    };

    return () => {
      socket.close();
    }

  }, []);

  return (
    <Canvas>
      <ambientLight intensity={0.5} />
      <pointLight position={[10, 10, 10]} />
      <Suspense fallback={null}>
        <Earth />
        <ISS position={coordinates} />
      </Suspense>
      <OrbitControls />
    </Canvas>
  )
}

export default App