import { useEffect, useMemo, useRef } from "react";
import { InstancedMesh, Object3D } from "three";

export function Debris({ data }: { data: [number, number, number][] }) {
    const meshRef = useRef<InstancedMesh | null>(null);
    const dummy = useMemo(() => new Object3D(), []);

    useEffect(() => {
        data.forEach((coords: [number, number, number], index: number) => {
            if (meshRef.current) {
                dummy.position.set(coords[0] / 6371, coords[2] / 6371, coords[1] / 6371);
                dummy.updateMatrix();
                
                meshRef.current.setMatrixAt(index, dummy.matrix);
            }
        });

        if (meshRef.current) {
            meshRef.current.instanceMatrix.needsUpdate = true;
        }

    }, [dummy, data]);

    return (
        <instancedMesh ref={meshRef} args={[undefined, undefined, 250]}>
            <boxGeometry args={[0.01, 0.01, 0.01]} />
            <meshBasicMaterial color="red" />
        </instancedMesh>
    );
}