INSERT INTO
  books(
    book_id,
    title,
    author,
    isbn,
    description,
    user_id,
    created_at,
    updated_at
  )
VALUES
  (
    '14c12190-ceb7-4e59-9f30-c98f3caeb473',
    '実践Rustプログラミング入門',
    '初田直也他',
    '978-4798061702',
    'C/C++の代わりとなるべき最新言語その独特な仕様をわかりやすく解説。',
    '5b4c96ac-316a-4bee-8e69-cac5eb84ff4c',
    now(),
    now()
  )
ON CONFLICT DO NOTHING;
