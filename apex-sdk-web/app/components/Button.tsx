import React, { ButtonHTMLAttributes } from 'react';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
    variant?: 'primary' | 'secondary' | 'glass';
    size?: 'sm' | 'md' | 'lg';
    fullWidth?: boolean;
}

export const Button = ({
    children,
    variant = 'primary',
    size = 'md',
    fullWidth = false,
    className = '',
    ...props
}: ButtonProps) => {
    const baseStyles = 'inline-flex items-center justify-center font-medium transition-all duration-200 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed';

    const variants = {
        primary: 'bg-vividOrange hover:bg-vividOrange-light text-white shadow-lg shadow-vividOrange/20',
        secondary: 'bg-obsidian-lighter hover:bg-slate-700 text-white',
        glass: 'bg-white/5 hover:bg-white/10 text-white backdrop-blur-sm border border-white/10',
    };

    const sizes = {
        sm: 'text-sm px-3 py-1.5 rounded-md',
        md: 'text-base px-6 py-3 rounded-lg',
        lg: 'text-lg px-8 py-4 rounded-xl',
    };

    const width = fullWidth ? 'w-full' : '';

    return (
        <button
            className={`${baseStyles} ${variants[variant]} ${sizes[size]} ${width} ${className}`}
            {...props}
        >
            {children}
        </button>
    );
};
