import { useGLTF, Center } from "@react-three/drei";

import { useMemo } from 'react';
import * as THREE from 'three';

export function Earth({ timestamp, isEcef }: { timestamp: number; isEcef: boolean }) {
    const { scene } = useGLTF('/earth.glb');
    
    const scale = useMemo(() => {
        const box = new THREE.Box3().setFromObject(scene);
        const size = new THREE.Vector3();
        box.getSize(size);
        const maxDim = Math.max(size.x, size.y, size.z);
        return 2 / maxDim;
    }, [scene]);

    const rotation = useMemo(() => {
        if (isEcef) return 0;

        const julianDate = timestamp / 86400 + 2440587.5;
        const centuriesSinceJ2000 = (julianDate - 2451545.0) / 36525;
        const gmstSeconds = 24110.54841 + 8640184.812866 * centuriesSinceJ2000
            + 0.093104 * centuriesSinceJ2000 ** 2;
        const siderealSeconds = (gmstSeconds + 1.00273790935 * (timestamp % 86400)) % 86400;
        return (siderealSeconds / 86400) * 2 * Math.PI;
    }, [timestamp, isEcef]);

    return (
        <Center rotation={[0, rotation, 0]}>
            <primitive object={scene} scale={scale} />
        </Center>
    )
}

export function ISS({ position, collisionDetected }: { position: [number, number, number]; collisionDetected: boolean }) {
    const { scene } = useGLTF('/iss.glb')
    
    return (
        <group position={position}>
            <primitive object={scene} scale={0.003} />
            
            <mesh>
                <sphereGeometry args={[0.025, 32, 32]} />
                <meshBasicMaterial color={collisionDetected ? "#ff0000" : "#00ffcc"} transparent opacity={0.3} depthWrite={false} />
            </mesh>
            
            <pointLight distance={1.5} intensity={2} color={collisionDetected ? "#ff0000" : "#00ffcc"} />
        </group>
    );
}
