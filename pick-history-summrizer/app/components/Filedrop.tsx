import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import path from 'path';

const FileDropZone = () => {
  const [filePath, setFilePath] = useState<string | null>(null);

  useEffect(() => {
    // Tauri のファイルドロップイベントをリッスン
    const unlisten = listen('tauri://file-drop', (event) => {
      const paths = event.payload as string[];
      if (paths && paths.length > 0) {
        console.log('File dropped:', paths[0]);
        setFilePath(paths[0]);  // ドロップされた最初のファイルパスを使用
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <div
      style={{
        border: '2px dashed #ccc',
        padding: '20px',
        textAlign: 'center',
      }}
    >
      {filePath ? (
        <p>ファイルパス: {filePath}</p>
      ) : (
        <p>ここにファイルをドラッグ＆ドロップしてください</p>
      )}
    </div>
  );
};

export default FileDropZone;
