import React, { useEffect, useRef } from "react";

interface LogDrawerProps {
  open: boolean;
  logs: string;
  onClose: () => void;
  autoScroll: boolean;
  setAutoScroll: (val: boolean) => void;
}

const LogDrawer: React.FC<LogDrawerProps> = ({ open, logs, onClose, autoScroll, setAutoScroll }) => {
  const contentRef = useRef<HTMLPreElement | null>(null);

  // Auto-scroll na dół przy zmianie logów jeśli szuflada otwarta
  useEffect(() => {
    if(open && autoScroll && contentRef.current){
      contentRef.current.scrollTop = contentRef.current.scrollHeight;
    }
  }, [logs, open, autoScroll]);

  return (
    <div className={`log-drawer${open ? " open" : ""}`}> 
      <div className="log-drawer-header">
        <span>Logi backendu</span>
        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
          <label style={{ fontSize: '0.7rem', display: 'flex', alignItems: 'center', gap: '4px' }}>
            <input type="checkbox" checked={autoScroll} onChange={e => setAutoScroll(e.target.checked)} /> auto scroll
          </label>
          <button className="log-drawer-close" onClick={onClose}>×</button>
        </div>
      </div>
      <pre ref={contentRef} className="log-drawer-content">{logs}</pre>
    </div>
  );
};

export default LogDrawer;
