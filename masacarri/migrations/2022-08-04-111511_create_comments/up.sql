CREATE TABLE comments (
  id UUID PRIMARY KEY,
  page_id UUID NOT NULL,
  reply_to UUID,
  ip_addr INET NOT NULL,
  display_name VARCHAR(1024) NOT NULL,
  site_url VARCHAR(1024),
  mail_addr VARCHAR(1024),
  content VARCHAR(16384) NOT NULL,
  delete_key VARCHAR(32) NOT NULL,
  flags INTEGER NOT NULL,
  created_time TIMESTAMP WITH TIME ZONE NOT NULL,
  FOREIGN KEY (page_id) REFERENCES pages (id),
  FOREIGN KEY (reply_to) REFERENCES comments (id)
);
