'use client';

import React, { useState } from 'react';

interface ProgressStep {
  id: string;
  title: string;
  description: string;
  completed: boolean;
  current: boolean;
}

interface ProgressTrackerProps {
  steps: ProgressStep[];
  className?: string;
}

export default function ProgressTracker({ steps, className = '' }: ProgressTrackerProps) {
  return (
    <div className={`bg-obsidian-light border border-obsidian-lighter rounded-lg p-6 ${className}`}>
      <h3 className="font-semibold text-white mb-4">Progress Tracker</h3>
      <div className="space-y-4">
        {steps.map((step, index) => (
          <div key={step.id} className="flex items-start space-x-4">
            {/* Step indicator */}
            <div className="flex-shrink-0">
              <div className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-semibold ${
                step.completed 
                  ? 'bg-green-500 text-white'
                  : step.current
                    ? 'bg-hyperBlue text-white'
                    : 'bg-obsidian-lighter text-slate-gray'
              }`}>
                {step.completed ? '✓' : index + 1}
              </div>
              {/* Connecting line */}
              {index < steps.length - 1 && (
                <div className={`w-0.5 h-6 mx-auto mt-1 ${
                  step.completed ? 'bg-green-500' : 'bg-obsidian-lighter'
                }`} />
              )}
            </div>
            
            {/* Step content */}
            <div className="flex-1 min-w-0">
              <h4 className={`font-medium ${
                step.current ? 'text-hyperBlue' : step.completed ? 'text-green-400' : 'text-white'
              }`}>
                {step.title}
              </h4>
              <p className="text-sm text-white/70 mt-1">
                {step.description}
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

// Quick Start Progress component
export function QuickStartProgress({ currentStep = 0 }: { currentStep?: number }) {
  const steps: ProgressStep[] = [
    {
      id: 'install',
      title: 'Installation',
      description: 'Set up your development environment and install Apex SDK',
      completed: currentStep > 0,
      current: currentStep === 0
    },
    {
      id: 'setup',
      title: 'Initial Setup',
      description: 'Configure your project and initialize the SDK',
      completed: currentStep > 1,
      current: currentStep === 1
    },
    {
      id: 'first-app',
      title: 'Build Your First App',
      description: 'Create a simple application using Apex SDK',
      completed: currentStep > 2,
      current: currentStep === 2
    },
    {
      id: 'deploy',
      title: 'Deploy & Test',
      description: 'Deploy your application and test on a testnet',
      completed: currentStep > 3,
      current: currentStep === 3
    }
  ];

  return <ProgressTracker steps={steps} />;
}