import { useEffect, useMemo, useRef } from "react";
import * as THREE from "three";
import { useGLTF } from "@react-three/drei";

export function Debris({ data }: { data: [number, number, number][] }) {
    const meshRef = useRef<THREE.InstancedMesh>(null);
    const dummy = useMemo(() => new THREE.Object3D(), []);
    
    const { scene } = useGLTF('/asteroid.glb');

    const asteroidGeometry = useMemo(() => {
        let geo: THREE.BufferGeometry | null = null;
        let mat: THREE.Material | THREE.Material[] | null = null;
        scene.traverse((child) => {
            if ((child as THREE.Mesh).isMesh) {
                geo = (child as THREE.Mesh).geometry;
                mat = (child as THREE.Mesh).material;
            }
        });
        return { geo, mat };
    }, [scene]);

    useEffect(() => {
        if (!meshRef.current || !asteroidGeometry) return;

        data.forEach((coords, index) => {
            dummy.position.set(coords[0] / 6371, coords[2] / 6371, coords[1] / 6371);
            
            dummy.scale.set(0.01, 0.01, 0.01);
            dummy.updateMatrix();
            
            meshRef.current!.setMatrixAt(index, dummy.matrix);
        });

        meshRef.current.instanceMatrix.needsUpdate = true;
    }, [dummy, data, asteroidGeometry]);

    if (!asteroidGeometry.geo || !asteroidGeometry.mat) return null;

    return (
        <instancedMesh ref={meshRef} args={[asteroidGeometry.geo, asteroidGeometry.mat, data.length]}></instancedMesh>
    );
}