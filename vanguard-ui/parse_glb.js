import fs from 'fs';

function getGLBBoundingBox(filePath) {
    const buffer = fs.readFileSync(filePath);
    
    const magic = buffer.toString('utf8', 0, 4);
    if (magic !== 'glTF') throw new Error('Not a GLB file');
    
    const jsonChunkLength = buffer.readUInt32LE(12);
    const jsonChunkType = buffer.toString('utf8', 16, 20);
    if (jsonChunkType !== 'JSON') throw new Error('First chunk is not JSON');
    
    const jsonString = buffer.toString('utf8', 20, 20 + jsonChunkLength);
    const gltf = JSON.parse(jsonString);
    
    // Find all meshes -> primitives -> attributes -> POSITION -> accessors -> min/max
    let minX = Infinity, minY = Infinity, minZ = Infinity;
    let maxX = -Infinity, maxY = -Infinity, maxZ = -Infinity;
    
    for (const mesh of gltf.meshes || []) {
        for (const prim of mesh.primitives || []) {
            const posAccessorIdx = prim.attributes.POSITION;
            if (posAccessorIdx !== undefined) {
                const accessor = gltf.accessors[posAccessorIdx];
                if (accessor.min && accessor.max) {
                    minX = Math.min(minX, accessor.min[0]);
                    minY = Math.min(minY, accessor.min[1]);
                    minZ = Math.min(minZ, accessor.min[2]);
                    maxX = Math.max(maxX, accessor.max[0]);
                    maxY = Math.max(maxY, accessor.max[1]);
                    maxZ = Math.max(maxZ, accessor.max[2]);
                }
            }
        }
    }
    
    const sizeX = maxX - minX;
    const sizeY = maxY - minY;
    const sizeZ = maxZ - minZ;
    
    return { sizeX, sizeY, sizeZ, maxDim: Math.max(sizeX, sizeY, sizeZ) };
}

const issBox = getGLBBoundingBox('public/iss.glb');
console.log('ISS Box:', issBox);
