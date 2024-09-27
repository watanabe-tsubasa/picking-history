import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';

interface FileDropZoneProps {
  filePath: string | null;
  errorMessage: string | null;
}

const FileDropZone: React.FC<FileDropZoneProps> = ({ filePath, errorMessage }) => {
  const [isHovered, setIsHovered] = useState(false);

  useEffect(() => {
    const unlistenDropHover = listen('tauri://file-drop-hover', () => {
      setIsHovered(true);
      console.log('File Hovered');
    });

    const unlistenDropCancelled = listen('tauri://file-drop-cancelled', () => {
      setIsHovered(false); 
      console.log('File Drop Cancelled');
    });

    const unlistenDrop = listen('tauri://file-drop', (event) => {
      const paths = event.payload as string[];
      if (paths && paths.length > 0) {
        setIsHovered(false);
        console.log('File Dropped:', paths[0]);
      }
    });

    // クリーンアップ
    return () => {
      unlistenDropHover.then((unlisten) => unlisten());
      unlistenDropCancelled.then((unlisten) => unlisten());
      unlistenDrop.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <div
      className={`border-2 border-dashed px-10 py-4 text-center w-full transition-colors ${
        isHovered ? 'border-gray-500 bg-gray-100' : 'border-gray-300'
      }`}
    >
      <div className='min-h-24 flex flex-col justify-center items-center'>
        {filePath ? (
          <p>{filePath}</p>
        ) : (
          <p>ここにファイルをドラッグ＆ドロップしてください</p>
        )}
        {errorMessage && (
          <p className="text-red-500 text-center pt-2">{errorMessage}</p>
        )}
      </div>
    </div>
  );
};

export default FileDropZone;
