document.addEventListener('DOMContentLoaded', () => {
    // Smooth scroll for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            document.querySelector(this.getAttribute('href')).scrollIntoView({
                behavior: 'smooth'
            });
        });
    });

    // Terminal animation
    const terminalBody = document.querySelector('.terminal-body');
    const terminalText = terminalBody.textContent;
    terminalBody.textContent = '';
    
    let i = 0;
    function typeTerminal() {
        if (i < terminalText.length) {
            terminalBody.textContent += terminalText.charAt(i);
            i++;
            setTimeout(typeTerminal, 50);
        }
    }
    typeTerminal();

    // Intersection Observer for animations
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = 1;
                entry.target.style.transform = 'translateY(0)';
            }
        });
    });

    // Observe feature cards and terminal
    document.querySelectorAll('.feature-card, .terminal-window').forEach(el => {
        el.style.opacity = 0;
        el.style.transform = 'translateY(20px)';
        el.style.transition = 'all 0.6s ease-out';
        observer.observe(el);
    });

    // Copy to clipboard functionality
    const codeBlock = document.querySelector('.code-block');
    codeBlock.addEventListener('click', () => {
        const text = codeBlock.textContent.replace('▋', '');
        navigator.clipboard.writeText(text).then(() => {
            const feedback = document.createElement('div');
            feedback.textContent = 'Copied to clipboard!';
            feedback.style.position = 'absolute';
            feedback.style.bottom = '-30px';
            feedback.style.left = '50%';
            feedback.style.transform = 'translateX(-50%)';
            feedback.style.color = '#10b981';
            codeBlock.appendChild(feedback);
            
            setTimeout(() => feedback.remove(), 2000);
        });
    });
});