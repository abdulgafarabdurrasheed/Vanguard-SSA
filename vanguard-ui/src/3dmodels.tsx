import { useGLTF, Center } from "@react-three/drei";

import { useMemo } from 'react';
import * as THREE from 'three';

export function Earth() {
    const { scene } = useGLTF('/earth.glb');
    
    const scale = useMemo(() => {
        const box = new THREE.Box3().setFromObject(scene);
        const size = new THREE.Vector3();
        box.getSize(size);
        const maxDim = Math.max(size.x, size.y, size.z);
        return 2 / maxDim;
    }, [scene]);

    return (
        <Center>
            <primitive object={scene} scale={scale} />
        </Center>
    )
}

export function ISS({ position }: { position: [number, number, number] }) {
    const { scene } = useGLTF('/iss.glb')
    
    return (
        <group position={position}>
            <primitive object={scene} scale={0.003} />
            
            <mesh>
                <sphereGeometry args={[0.025, 32, 32]} />
                <meshBasicMaterial color="#00ffcc" transparent opacity={0.3} depthWrite={false} />
            </mesh>
            
            <pointLight distance={1.5} intensity={2} color="#00ffcc" />
        </group>
    );
}

export function Asteroid({ position }: { position: [number, number, number] }) {
    const { scene } = useGLTF('/asteroid.glb');

    return (
        <group position={position}>
            <primitive object={scene} scale={0.0005} />
            
            <mesh>
                <sphereGeometry args={[0.025, 32, 32]} />
                <meshBasicMaterial color="#ffcc00" transparent opacity={0.3} depthWrite={false} />
            </mesh>
            
            <pointLight distance={1.5} intensity={2} color="#ffcc00" />
        </group>
    );
}