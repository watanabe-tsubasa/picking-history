import React, { useEffect, useState } from 'react';
import FileDropZone from './Filedrop';
import { listen } from '@tauri-apps/api/event';
import { dirname, join } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';
import { Loader2 } from 'lucide-react';
import { toast, Toaster } from 'sonner';

const FileHandler = () => {
  const [filePath, setFilePath] = useState<string | null>(null);
  const [outputPath, setOutputPath] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState<boolean>(false);

  const handleUploadButtonClick = async () => {
    try {
      const path = await invoke<string>('get_file_path');
      validateFile(path);
    } catch (error) {
      console.error(error);
    }
  };

  const validateFile = async (path: string) => {
    const extension = path.split('.').pop()?.toLowerCase();
    if (
      // extension === 'csv' ||
      extension === 'xlsx'
    ) {
      setFilePath(path);
      const dir = await dirname(path as string);
      const resultPath = await join(dir, 'output.xlsx');
      setOutputPath(resultPath);
      setErrorMessage(null);
    } else {
      setErrorMessage('対応しているファイル形式は XLSX です');
      setFilePath(null);
    }
  };

  const handleOutputButtonClick = async () => {
    setIsGenerating(true);
    try {
      const res = await invoke('process_excel', { inputFile: filePath, outputFile: outputPath });
      console.log(`res: ${res}`);
      toast.success('ファイルの生成に成功しました')
    } catch (error) {
      console.error(error);
      toast.error(`読み込むファイルを確認してください：${error}`)
    } finally{
      setIsGenerating(false);
      setFilePath(null);
      setOutputPath(null);
    }
  }

  useEffect(() => {
    const unlisten = listen('tauri://file-drop', (event) => {
      const paths = event.payload as string[];
      if (paths && paths.length > 0) {
        console.log('File dropped:', paths[0]);
        validateFile(paths[0]);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="w-full">
      <FileDropZone filePath={filePath} errorMessage={errorMessage} />      
      <div className="flex flex-row items-center justify-center w-full">
        <div className="flex gap-4 items-center flex-col sm:flex-row pt-5">
          <button
            className="rounded-full border border-solid border-black/[.08] dark:border-white/[.145] transition-colors flex items-center justify-center hover:bg-[#f2f2f2] dark:hover:bg-[#1a1a1a] hover:border-transparent text-sm sm:text-base h-10 sm:h-12 px-4 sm:px-5 min-w-44"
            onClick={handleUploadButtonClick}
          >
            アップロード
          </button>
          <button
            className={`rounded-full border border-solid border-transparent transition-colors flex items-center justify-center text-sm sm:text-base h-10 sm:h-12 px-4 min-w-44
              ${filePath
                ? 'bg-foreground text-background gap-2 hover:bg-[#383838] dark:hover:bg-[#ccc]'
                : 'bg-[#ddd] text-[#aaa] cursor-not-allowed'}
            `}
            onClick={handleOutputButtonClick}
            disabled={!filePath}
          >
            {!isGenerating
              ? 'ファイル出力'
              : <Loader2 className='animate-spin' />
            }
          </button>
          <Toaster duration={3000} />
        </div>
      </div>
    </div>
  );
};

export default FileHandler;
