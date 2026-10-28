import { Canvas } from '@react-three/fiber'
import { OrbitControls } from '@react-three/drei'
import './index.css'
import { useState, useEffect, Suspense, useRef } from 'react'
import { ISS } from './3dmodels';
import { Earth } from './3dmodels';
import { Debris, Satellites } from './debris';

function App() {
  const [coordinates, setCoordinates] = useState<[number, number, number]>([0, 0, 0]);
  const [debrisData, setDebrisData] = useState<[number, number, number][]>([]);
  const [activeSatellite, setActiveSatellite] = useState<[number, number, number][]>([]);
  const [collisionDetected, setCollisionDetected] = useState(false);
  const [timestamp, setTimestamp] = useState<number>(0);

  const [multiplier, setMultiplier] = useState(60.0);
  const [isEcef, setIsEcef] = useState(true);

  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const backendUrl = import.meta.env.VITE_BACKEND_URL?.trim() || window.location.origin;
    const socketUrl = new URL('/ws', backendUrl);
    socketUrl.protocol = socketUrl.protocol === 'https:' ? 'wss:' : 'ws:';
    const socket = new WebSocket(socketUrl);
    socketRef.current = socket;
    
    socket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setCoordinates([
        data.predicted_xyz[0] / 6371, 
        data.predicted_xyz[2] / 6371, 
        data.predicted_xyz[1] / 6371
      ]);
      setCollisionDetected(data.collision_warning);
      setDebrisData(data.debris_field);
      setActiveSatellite(data.active_satellite);
      if (data.universal_timestamp) setTimestamp(data.universal_timestamp);
    };

    return () => {
      if (socketRef.current && socketRef.current.readyState === WebSocket.OPEN) {
        socketRef.current.close();
      }
    }
  }, []);

  const updateConfig = (newMultiplier: number, newEcef: boolean) => {
    setMultiplier(newMultiplier);
    setIsEcef(newEcef);
    if (socketRef.current && socketRef.current.readyState === WebSocket.OPEN) {
      socketRef.current.send(JSON.stringify({
        time_multiplier: newMultiplier,
        is_ecef_frame: newEcef
      }));
    }
  };

  return (
    <>
      <div style={{ position: 'absolute', top: 20, left: 20, zIndex: 10, background: 'rgba(0,0,0,0.7)', padding: '15px', color: 'white', borderRadius: '8px', fontFamily: 'sans-serif' }}>
        <div style={{ marginBottom: '10px' }}>
          <label style={{ marginRight: '10px' }}>Time Speed:</label>
          <select value={multiplier} onChange={(e) => updateConfig(Number(e.target.value), isEcef)} style={{ background: '#333', color: 'white', border: 'none', padding: '5px' }}>
            <option value={1}>1x (Realtime)</option>
            <option value={60}>60x (1 min/sec)</option>
            <option value={3600}>3600x (1 hr/sec)</option>
            <option value={86400}>86400x (1 day/sec)</option>
          </select>
        </div>
        <div>
          <label style={{ marginRight: '10px' }}>Reference Frame:</label>
          <button onClick={() => updateConfig(multiplier, !isEcef)} style={{ background: isEcef ? '#4CAF50' : '#2196F3', color: 'white', border: 'none', padding: '5px 10px', cursor: 'pointer', borderRadius: '4px' }}>
            {isEcef ? "ECEF (Static Earth)" : "ECI (Spinning Earth)"}
          </button>
        </div>
      </div>

      <Canvas>
        <ambientLight intensity={0.5} />
        <pointLight position={[10, 10, 10]} />
        <Suspense fallback={null}>
          <Earth timestamp={timestamp} isEcef={isEcef} />
          <ISS position={coordinates} collisionDetected={collisionDetected} />
          <Satellites data={activeSatellite} />
          <Debris data={debrisData} />
        </Suspense>
        <OrbitControls />
      </Canvas>
    </>
  )
}

export default App
