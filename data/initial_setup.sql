INSERT INTO
  roles(name)
VALUES
  ('Admin'),
  ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
  users(name, email, password_hash, role_id)
SELECT
  'Eleazar Fig',
  'eleazar.fig@example.com',
  '$2b$12$nQntihUyqiFlV4i3O2QxM.msFL6j4./n1yrXEglIePMaj1KGV0A12',
  role_id
FROM
  roles
WHERE
  name LIKE 'Admin';
