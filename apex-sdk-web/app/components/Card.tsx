import React from 'react';

interface CardProps {
    title: string;
    description: string;
    icon?: React.ReactNode;
    className?: string;
}

export const Card = ({ title, description, icon, className = '' }: CardProps) => {
    return (
        <div className={`p-6 bg-obsidian-light border border-obsidian-lighter rounded-xl hover:border-hyperBlue/50 hover:shadow-lg hover:shadow-hyperBlue/10 transition-all duration-200 transform hover:scale-[1.02] cursor-pointer group ${className}`}>
            {icon && (
                <div className="mb-4 text-hyperBlue group-hover:text-hyperBlue-hover transition-colors">
                    {icon}
                </div>
            )}
            <h3 className="text-xl font-semibold text-white mb-2 group-hover:text-hyperBlue transition-colors">
                {title}
            </h3>
            <p className="text-slate-gray leading-relaxed">
                {description}
            </p>
        </div>
    );
};
