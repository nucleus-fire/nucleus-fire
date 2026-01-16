-- Seed default email templates
INSERT INTO email_templates (name, subject, body, created_at)
VALUES 
    ('Welcome Email', 'Welcome to our Newsletter!', '<h1>Welcome!</h1><p>Thanks for joining us.</p>', datetime('now')),
    ('Monthly Update', 'Your Monthly Digest', '<h1>Monthly Update</h1><p>Here is what happened this month...</p>', datetime('now'))
ON CONFLICT DO NOTHING;
