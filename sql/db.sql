--
-- PostgreSQL database dump
--

-- Dumped from database version 14.1
-- Dumped by pg_dump version 14.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: authorizations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.authorizations (
    id integer NOT NULL,
    user_id integer NOT NULL,
    uuid uuid NOT NULL,
    client_type smallint DEFAULT 0 NOT NULL,
    refresh_token uuid NOT NULL,
    create_time timestamp with time zone NOT NULL,
    update_time timestamp with time zone,
    last_refresh_time timestamp with time zone,
    access_token_id uuid NOT NULL,
    access_token_exp timestamp with time zone NOT NULL,
    access_token_iat timestamp with time zone NOT NULL,
    is_enabled smallint DEFAULT 1 NOT NULL
);


ALTER TABLE public.authorizations OWNER TO postgres;

--
-- Name: authorizations_blacklist; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.authorizations_blacklist (
    id integer NOT NULL,
    access_token_id uuid NOT NULL,
    access_token_exp timestamp with time zone NOT NULL,
    user_id integer DEFAULT 0 NOT NULL
);


ALTER TABLE public.authorizations_blacklist OWNER TO postgres;

--
-- Name: authorizations_blacklist_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.authorizations_blacklist_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.authorizations_blacklist_id_seq OWNER TO postgres;

--
-- Name: authorizations_blacklist_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.authorizations_blacklist_id_seq OWNED BY public.authorizations_blacklist.id;


--
-- Name: authorizations_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.authorizations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.authorizations_id_seq OWNER TO postgres;

--
-- Name: authorizations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.authorizations_id_seq OWNED BY public.authorizations.id;


--
-- Name: authorizations_logs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.authorizations_logs (
    id integer NOT NULL,
    user_id integer,
    log_type smallint NOT NULL,
    ip character varying(15),
    log_time timestamp with time zone NOT NULL,
    client_type smallint NOT NULL,
    auth_id integer,
    log character varying(250),
    user_agent text
);


ALTER TABLE public.authorizations_logs OWNER TO postgres;

--
-- Name: authorizations_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.authorizations_logs_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.authorizations_logs_id_seq OWNER TO postgres;

--
-- Name: authorizations_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.authorizations_logs_id_seq OWNED BY public.authorizations_logs.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id integer NOT NULL,
    uuid uuid NOT NULL,
    username character varying(50),
    password character varying(64),
    salt uuid,
    mobile character varying(11),
    create_time timestamp with time zone,
    update_time timestamp with time zone,
    is_del smallint DEFAULT 0,
    is_enabled smallint DEFAULT 1,
    last_login_time timestamp with time zone,
    last_login_ip character varying(15),
    user_type smallint DEFAULT 0,
    name character varying(50)
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: authorizations id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.authorizations ALTER COLUMN id SET DEFAULT nextval('public.authorizations_id_seq'::regclass);


--
-- Name: authorizations_blacklist id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.authorizations_blacklist ALTER COLUMN id SET DEFAULT nextval('public.authorizations_blacklist_id_seq'::regclass);


--
-- Name: authorizations_logs id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.authorizations_logs ALTER COLUMN id SET DEFAULT nextval('public.authorizations_logs_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: authorizations_blacklist authorizations_blacklist_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.authorizations_blacklist
    ADD CONSTRAINT authorizations_blacklist_pk PRIMARY KEY (id);


--
-- Name: authorizations_logs authorizations_logs_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.authorizations_logs
    ADD CONSTRAINT authorizations_logs_pk PRIMARY KEY (id);


--
-- Name: authorizations authorizations_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.authorizations
    ADD CONSTRAINT authorizations_pk PRIMARY KEY (id);


--
-- Name: users users_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pk PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

