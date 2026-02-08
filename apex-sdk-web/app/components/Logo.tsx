import React from 'react';
import Image from 'next/image';

export const Logo = ({ className = 'w-8 h-8' }: { className?: string }) => {
    return (
        <Image
            src="/assets/apex_logo.svg"
            alt="Apex SDK Logo"
            width={32}
            height={32}
            className={className}
            priority
        />
    );
};
