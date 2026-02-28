(function () {
  var STORAGE_KEY = 'jade-feedback';

  function getFeedback() {
    try {
      var raw = localStorage.getItem(STORAGE_KEY);
      var list = raw ? JSON.parse(raw) : [];
      return Array.isArray(list) ? list : [];
    } catch (e) {
      return [];
    }
  }

  function saveFeedback(list) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(list));
    } catch (e) {}
  }

  function addFeedback(title, type, message, email) {
    var list = getFeedback();
    list.unshift({
      id: 'f' + Date.now() + '-' + Math.random().toString(36).slice(2, 9),
      title: (title || '').trim() || 'No title',
      type: (type || 'other').trim(),
      message: (message || '').trim() || '',
      email: (email || '').trim() || '',
      createdAt: Date.now()
    });
    saveFeedback(list);
    return list[0];
  }

  function escapeHtml(s) {
    var div = document.createElement('div');
    div.textContent = s;
    return div.innerHTML;
  }

  function formatDate(ts) {
    var d = new Date(ts);
    return d.toLocaleDateString() + ' ' + d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }

  function typeLabel(t) {
    return { bug: 'Bug', feature: 'Feature request', other: 'Other' }[t] || t;
  }

  function renderRecent(container, limit) {
    limit = limit || 10;
    var list = getFeedback().slice(0, limit);
    if (!container) return;
    if (list.length === 0) {
      container.innerHTML = '<p class="feedback-empty">No submissions yet.</p>';
      return;
    }
    container.innerHTML = '<ul class="feedback-list">' + list.map(function (f) {
      return '<li class="feedback-item">' +
        '<span class="feedback-item-type">' + escapeHtml(typeLabel(f.type)) + '</span> ' +
        '<strong>' + escapeHtml(f.title) + '</strong> ' +
        '<span class="feedback-item-date">' + formatDate(f.createdAt) + '</span>' +
        (f.message ? '<p class="feedback-item-preview">' + escapeHtml(f.message.slice(0, 100)) + (f.message.length > 100 ? '…' : '') + '</p>' : '') +
      '</li>';
    }).join('') + '</ul>';
  }

  function init() {
    var form = document.getElementById('feedback-form');
    var successEl = document.getElementById('feedback-success');
    var recentEl = document.getElementById('feedback-recent');

    if (form) {
      form.onsubmit = function (e) {
        e.preventDefault();
        var title = (form.querySelector('[name="feedback-title"]') || {}).value;
        var type = (form.querySelector('[name="feedback-type"]') || {}).value;
        var message = (form.querySelector('[name="feedback-message"]') || {}).value;
        var email = (form.querySelector('[name="feedback-email"]') || {}).value;
        addFeedback(title, type, message, email);
        form.reset();
        if (successEl) {
          successEl.classList.remove('hidden');
          successEl.setAttribute('aria-hidden', 'false');
        }
        renderRecent(document.getElementById('feedback-recent'), 10);
      };
    }

    renderRecent(recentEl, 10);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
