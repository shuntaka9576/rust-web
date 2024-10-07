-- Add up migration script here
DROP TRIGGER IF EXISTS books_updated_at_trigger ON books;
DROP TABLE IF EXISTS books;

-- FIXME: 書籍ではIF EXISTSなしになっている
DROP FUNCTION IF EXISTS set_updated_at;
