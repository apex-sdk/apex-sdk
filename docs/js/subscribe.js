/* Newsletter subscription via Web3Forms */
(function(){
  const ENDPOINT = 'https://api.web3forms.com/submit';
  const ACCESS_KEY = '__WEB3FORMS_ACCESS_KEY__';

  function $(sel, root=document){ return root.querySelector(sel); }

  function init(form){
    if (!form) return;
    // Honeypot
    if (!form.querySelector('input[name="bot_field"]')){
      const hp = document.createElement('input');
      hp.type = 'text'; hp.name = 'bot_field'; hp.tabIndex = -1; hp.autocomplete = 'off';
      hp.style.position = 'absolute'; hp.style.left='-9999px'; hp.setAttribute('aria-hidden','true');
      form.appendChild(hp);
    }

    const emailInput = form.querySelector('input[type="email"], input[name="email"]');
    const submitBtn = form.querySelector('button[type="submit"], button, input[type="submit"]');
    let status = form.querySelector('.form-status');
    if (!status){
      status = document.createElement('div');
      status.className = 'form-status';
      status.setAttribute('aria-live','polite');
      status.style.marginTop = '.5rem';
      status.style.fontSize = '.9rem';
      status.style.color = 'var(--text-secondary)';
      form.appendChild(status);
    }

    form.addEventListener('submit', async (e) => {
      e.preventDefault();
      const email = (emailInput && emailInput.value || '').trim();
      if (!email || !/^\S+@\S+\.[\w-]{2,}$/.test(email)){
        status.textContent = 'Please enter a valid email address.';
        emailInput && emailInput.focus();
        return;
      }
      if (form.querySelector('input[name="bot_field"]').value){
        // Bot detected: pretend success
        status.textContent = 'Thanks for subscribing!';
        form.reset();
        return;
      }

      const source = form.getAttribute('data-source') || (location.pathname.includes('viewer') ? 'viewer' : 'homepage');
      const botFieldEl = form.querySelector('input[name="bot_field"]');

      const formData = new FormData();
      formData.append('access_key', ACCESS_KEY);
      formData.append('email', email);
      formData.append('subject', 'Apex SDK: New newsletter signup');
      formData.append('from_name', 'Apex SDK Website');
      formData.append('source', source);
      if (botFieldEl && botFieldEl.value) {
        formData.append('botcheck', botFieldEl.value);
      }

      try {
        if (submitBtn){ submitBtn.disabled = true; submitBtn.dataset._label = submitBtn.textContent; submitBtn.textContent = 'Subscribing…'; }
        status.textContent = 'Submitting…';
        const res = await fetch(ENDPOINT, {
          method: 'POST',
          body: formData
        });
        const data = await res.json().catch(()=>({ ok:false }));
        if ((data && data.ok) || res.ok){
          status.style.color = 'var(--success-color)';
          status.textContent = 'Thanks for subscribing! Please check your inbox to confirm.';
          form.reset();
        } else {
          status.style.color = 'var(--warning-color)';
          status.textContent = (data && (data.message||data.error)) || 'Subscription failed. Please try again later.';
        }
      } catch (err){
        status.style.color = 'var(--warning-color)';
        status.textContent = 'Network error. Please try again.';
      } finally {
        if (submitBtn){ submitBtn.disabled = false; submitBtn.textContent = submitBtn.dataset._label || 'Subscribe'; }
      }
    });
  }

  function initAll(){
    document.querySelectorAll('form[data-subscribe]')
      .forEach(init);
  }

  if (document.readyState === 'loading'){
    document.addEventListener('DOMContentLoaded', initAll);
  } else {
    initAll();
  }
})();
