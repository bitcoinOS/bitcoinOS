import { useState, useEffect } from 'react';

function useIsScrolledToBottom() {
    const [isScrolledToBottom, setIsScrolledToBottom] = useState(false);

    useEffect(() => {
        const handleScroll = () => {
            const scrollPosition = window.innerHeight + window.scrollY;
            const bodyHeight = document.body.offsetHeight;
            const offset = 100;
            setIsScrolledToBottom(scrollPosition > bodyHeight - offset);
        };

        window.addEventListener('scroll', handleScroll);
        return () => window.removeEventListener('scroll', handleScroll);
    }, []);

    return isScrolledToBottom;
}

export { useIsScrolledToBottom }