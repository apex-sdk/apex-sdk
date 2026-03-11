'use client';

import React, { useState } from 'react';
import { Button } from './Button';

const MOCK_OUTPUT = [
    '> Compiling apex-sdk v0.1.0...',
    '> Finished dev [unoptimized + debuginfo] target(s) in 0.8s',
    '> Running `target/debug/quickstart`',
    '',
    'Connected to Apex Mainnet (ID: 101)',
    'Client Version: 1.4.2-rust',
    'Block Height: 12,450,291',
    '-----------------------------------------',
    'Client initialized successfully.',
];

export const LiveTerminal = () => {
    const [isOpen, setIsOpen] = useState(false);
    const [output, setOutput] = useState<string[]>([]);
    const [isRunning, setIsRunning] = useState(false);

    const handleRun = () => {
        setIsOpen(true);
        setIsRunning(true);
        setOutput([]);

        // Simulate compilation and execution delay
        let currentLine = 0;
        const interval = setInterval(() => {
            if (currentLine >= MOCK_OUTPUT.length) {
                clearInterval(interval);
                setIsRunning(false);
                return;
            }
            setOutput(prev => [...prev, MOCK_OUTPUT[currentLine]]);
            currentLine++;
        }, 400); // 400ms per line
    };

    return (
        <div className="w-full max-w-4xl mx-auto rounded-xl overflow-hidden shadow-2xl bg-[#0F172A] border border-obsidian-lighter flex flex-col md:flex-row relative">
            {/* Code Side */}
            <div className="flex-1 p-6 font-mono text-sm overflow-x-auto relative z-10 bg-[#0F172A]">
                <div className="flex justify-between items-start mb-4">
                    <div className="flex space-x-2">
                        <div className="w-3 h-3 rounded-full bg-red-500/20 text-red-500 flex items-center justify-center">●</div>
                        <div className="w-3 h-3 rounded-full bg-yellow-500/20 text-yellow-500 flex items-center justify-center">●</div>
                        <div className="w-3 h-3 rounded-full bg-green-500/20 text-green-500 flex items-center justify-center">●</div>
                    </div>
                    <span className="text-xs text-slate-500">main.rs</span>
                </div>
                <pre className="text-slate-300 leading-relaxed">
                    <span className="text-pink-400">use</span> apex_sdk::prelude::*;<br /><br />
                    <span className="text-pink-400">#[tokio::main]</span><br />
                    <span className="text-hyperBlue">async fn</span> <span className="text-yellow-300">main</span>() -&gt; Result&lt;()&gt; {'{'}<br />
                    {'    '}<span className="text-slate-500">// Initialize client</span><br />
                    {'    '}<span className="text-pin-400">let</span> client = <span className="text-yellow-300">ApexClient</span>::builder()<br />
                    {'        '}.network(Network::Mainnet)<br />
                    {'        '}.build()?;<br /><br />
                    {'    '}println!(<span className="text-green-300">"Connected: {'{:?}'}"</span>, client.version());<br />
                    {'    '}Ok(())<br />
                    {'}'}
                </pre>
                <div className="mt-8">
                    <Button variant="primary" size="sm" onClick={handleRun} disabled={isRunning}>
                        {isRunning ? 'Building...' : 'Run in Browser'}
                    </Button>
                </div>
            </div>

            {/* Terminal Drawer Side */}
            <div
                className={`absolute inset-y-0 right-0 w-full md:w-1/2 bg-black/90 backdrop-blur-md border-l border-obsidian-lighter transition-transform duration-300 ease-in-out z-20 ${isOpen ? 'translate-x-0' : 'translate-x-full'}`}
            >
                <div className="h-full flex flex-col p-4 font-mono text-xs md:text-sm">
                    <div className="flex justify-between items-center mb-4 border-b border-white/10 pb-2">
                        <span className="text-slate-400">Terminal Output</span>
                        <button onClick={() => setIsOpen(false)} className="text-slate-500 hover:text-white">×</button>
                    </div>

                    <div className="flex-1 overflow-y-auto space-y-1">
                        {output.map((line, i) => (
                            <div key={i} className={`${line.startsWith('>') ? 'text-slate-500' : 'text-green-400'} animate-fade-in`}>
                                {line}
                            </div>
                        ))}
                        {isRunning && <div className="text-hyperBlue animate-pulse">_</div>}
                    </div>
                </div>
            </div>
        </div>
    );
};
