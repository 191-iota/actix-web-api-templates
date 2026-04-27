create table if not exists public.bookmark (
    id uuid primary key default gen_random_uuid(),
    url text not null,
    host text not null,
    title text,
    created_at timestamptz not null default now()
);
