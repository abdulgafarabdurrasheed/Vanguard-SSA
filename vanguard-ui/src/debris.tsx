import { useEffect, useMemo, useRef } from "react";
import * as THREE from "three";
import { useGLTF } from "@react-three/drei";

export function Debris({ data }: { data: [number, number, number][] }) {
    const meshRef = useRef<THREE.InstancedMesh>(null);
    const dummy = useMemo(() => new THREE.Object3D(), []);
    
    const { scene } = useGLTF('/asteroid.glb');

    const asteroidGeometry = useMemo(() => {
        let geo: THREE.BufferGeometry | null = null;
        scene.traverse((child) => {
            if ((child as THREE.Mesh).isMesh) {
                geo = (child as THREE.Mesh).geometry;
            }
        });
        return geo;
    }, [scene]);

    useEffect(() => {
        if (!meshRef.current || !asteroidGeometry) return;

        data.forEach((coords, index) => {
            dummy.position.set(coords[0] / 6371, coords[2] / 6371, coords[1] / 6371);
            
            dummy.scale.set(0.005, 0.005, 0.005);
            dummy.updateMatrix();
            
            meshRef.current!.setMatrixAt(index, dummy.matrix);
        });

        meshRef.current.instanceMatrix.needsUpdate = true;
    }, [dummy, data, asteroidGeometry]);

    if (!asteroidGeometry) return null;

    return (
        <instancedMesh ref={meshRef} args={[asteroidGeometry, undefined, data.length]}>
            <meshStandardMaterial color="#888888" roughness={0.8} />
        </instancedMesh>
    );
}