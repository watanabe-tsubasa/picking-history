import React, { useState } from 'react';

const FileDropZone = () => {
  const [filePath, setFilePath] = useState<string | null>(null);

  const handleDrop = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    const file = event.dataTransfer.files[0];
    if (file) {
      // ファイルパスをセット
      setFilePath(file.path);
    }
  };

  const handleDragOver = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
  };

  return (
    <div
      onDrop={handleDrop}
      onDragOver={handleDragOver}
      style={{
        border: '2px dashed #ccc',
        padding: '20px',
        textAlign: 'center',
      }}
    >
      {filePath ? (
        <p>ファイルパス: {filePath}</p>
      ) : (
        <p>ここにファイルをドロップしてください</p>
      )}
    </div>
  );
};

export default FileDropZone;
