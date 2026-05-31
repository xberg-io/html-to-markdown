const { convert } = require('./crates/html-to-markdown-node');

const html = '<h1>Title</h1><p><a href="https://example.com">link</a></p>';
const called = [];

const result = convert(html, {
  visitor: {
    visitLink() { called.push('visitLink'); return 'continue'; },
    visitHeading() { called.push('visitHeading'); return 'continue'; },
    visitText() { called.push('visitText'); return 'continue'; },
  },
});

console.log('Callbacks fired:', called.length > 0 ? called.join(', ') : 'NONE');
console.log('Content length:', result.content.length);
