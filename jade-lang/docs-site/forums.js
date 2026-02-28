(function () {
  var STORAGE_KEY = 'jade-forums';

  function getTopics() {
    try {
      var raw = localStorage.getItem(STORAGE_KEY);
      var data = raw ? JSON.parse(raw) : { topics: [] };
      if (!Array.isArray(data.topics)) data.topics = [];
      return data.topics;
    } catch (e) {
      return [];
    }
  }

  function saveTopics(topics) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify({ topics: topics }));
    } catch (e) {}
  }

  function id() {
    return 't' + Date.now() + '-' + Math.random().toString(36).slice(2, 9);
  }

  function addTopic(title, author, body) {
    var topics = getTopics();
    topics.unshift({
      id: id(),
      title: (title || '').trim() || 'Untitled',
      author: (author || '').trim() || 'Anonymous',
      body: (body || '').trim() || '',
      createdAt: Date.now(),
      replies: []
    });
    saveTopics(topics);
    return topics[0];
  }

  function addReply(topicId, author, body) {
    var topics = getTopics();
    var topic = topics.find(function (t) { return t.id === topicId; });
    if (!topic) return null;
    if (!Array.isArray(topic.replies)) topic.replies = [];
    topic.replies.push({
      id: id(),
      author: (author || '').trim() || 'Anonymous',
      body: (body || '').trim() || '',
      createdAt: Date.now()
    });
    saveTopics(topics);
    return topic;
  }

  function escapeHtml(s) {
    var div = document.createElement('div');
    div.textContent = s;
    return div.innerHTML;
  }

  function formatDate(ts) {
    var d = new Date(ts);
    var now = new Date();
    var diff = now - d;
    if (diff < 60000) return 'Just now';
    if (diff < 3600000) return Math.floor(diff / 60000) + 'm ago';
    if (diff < 86400000) return Math.floor(diff / 3600000) + 'h ago';
    return d.toLocaleDateString() + ' ' + d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }

  function renderTopicList(container) {
    var topics = getTopics();
    if (!container) return;
    if (topics.length === 0) {
      container.innerHTML = '<p class="forum-empty">No topics yet. Start one below.</p>';
      return;
    }
    container.innerHTML = topics.map(function (t) {
      var replyCount = (t.replies && t.replies.length) || 0;
      return (
        '<article class="forum-topic-card" data-topic-id="' + escapeHtml(t.id) + '">' +
          '<h3 class="forum-topic-title">' + escapeHtml(t.title) + '</h3>' +
          '<p class="forum-topic-meta">' + escapeHtml(t.author) + ' · ' + formatDate(t.createdAt) + (replyCount ? ' · ' + replyCount + ' reply' + (replyCount !== 1 ? 'ies' : '') : '') + '</p>' +
          '<p class="forum-topic-preview">' + escapeHtml((t.body || '').slice(0, 120)) + (t.body && t.body.length > 120 ? '…' : '') + '</p>' +
        '</article>'
      );
    }).join('');
    container.querySelectorAll('.forum-topic-card').forEach(function (el) {
      el.addEventListener('click', function () {
        showThread(el.getAttribute('data-topic-id'));
      });
    });
  }

  function showThread(topicId) {
    var topics = getTopics();
    var topic = topics.find(function (t) { return t.id === topicId; });
    if (!topic) return;
    var listEl = document.getElementById('forum-topic-list-wrap');
    var threadEl = document.getElementById('forum-thread');
    var threadTitle = document.getElementById('forum-thread-title');
    var threadBody = document.getElementById('forum-thread-body');
    var threadReplies = document.getElementById('forum-thread-replies');
    var replyForm = document.getElementById('forum-reply-form');
    if (listEl) listEl.classList.add('hidden');
    if (threadEl) threadEl.classList.remove('hidden');
    if (threadTitle) threadTitle.textContent = topic.title;
    if (threadBody) {
      threadBody.innerHTML = '<p class="forum-thread-meta">' + escapeHtml(topic.author) + ' · ' + formatDate(topic.createdAt) + '</p><div class="forum-thread-body">' + escapeHtml(topic.body).replace(/\n/g, '<br>') + '</div>';
    }
    if (threadReplies) {
      var replies = topic.replies || [];
      threadReplies.innerHTML = replies.length === 0
        ? '<p class="forum-empty">No replies yet.</p>'
        : replies.map(function (r) {
            return '<div class="forum-reply"><p class="forum-reply-meta">' + escapeHtml(r.author) + ' · ' + formatDate(r.createdAt) + '</p><p class="forum-reply-body">' + escapeHtml(r.body).replace(/\n/g, '<br>') + '</p></div>';
          }).join('');
    }
    if (replyForm) {
      replyForm.onsubmit = null;
      replyForm.dataset.topicId = topicId;
      replyForm.onsubmit = function (e) {
        e.preventDefault();
        var author = (replyForm.querySelector('[name="reply-author"]') || {}).value;
        var body = (replyForm.querySelector('[name="reply-body"]') || {}).value;
        addReply(topicId, author, body);
        replyForm.querySelector('[name="reply-body"]').value = '';
        showThread(topicId);
      };
    }
  }

  function backToList() {
    document.getElementById('forum-topic-list-wrap').classList.remove('hidden');
    document.getElementById('forum-thread').classList.add('hidden');
    renderTopicList(document.getElementById('forum-topic-list'));
  }

  function init() {
    var container = document.getElementById('forum-topic-list');
    renderTopicList(container);

    var newForm = document.getElementById('forum-new-topic-form');
    if (newForm) {
      newForm.onsubmit = function (e) {
        e.preventDefault();
        var title = (newForm.querySelector('[name="topic-title"]') || {}).value;
        var author = (newForm.querySelector('[name="topic-author"]') || {}).value;
        var body = (newForm.querySelector('[name="topic-body"]') || {}).value;
        addTopic(title, author, body);
        newForm.querySelector('[name="topic-title"]').value = '';
        newForm.querySelector('[name="topic-body"]').value = '';
        renderTopicList(document.getElementById('forum-topic-list'));
      };
    }

    var backBtn = document.getElementById('forum-back-btn');
    if (backBtn) backBtn.addEventListener('click', backToList);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
