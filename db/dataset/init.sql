-- public.countries definition

-- Drop table

-- DROP TABLE public.countries;

CREATE TABLE public.countries (
	iso3166 varchar NOT NULL,
	"name" varchar NULL,
	CONSTRAINT countries_pkey_1 PRIMARY KEY (iso3166)
);


-- public.languages definition

-- Drop table

-- DROP TABLE public.languages;

CREATE TABLE public.languages (
	iso693_3 varchar(3) NOT NULL,
	"name" varchar NULL,
	CONSTRAINT languages_pkey PRIMARY KEY (iso693_3)
);


-- public.authors definition

-- Drop table

-- DROP TABLE public.authors;

CREATE TABLE public.authors (
	id varchar DEFAULT gen_random_uuid() NOT NULL,
	last_name varchar NULL,
	first_name varchar NULL,
	pseudonym varchar NULL,
	country1 varchar NULL,
	birthyear int4 NULL,
	country2 varchar NULL,
	CONSTRAINT authors_pkey PRIMARY KEY (id)
);

INSERT INTO public.authors (id,last_name,first_name,pseudonym,country1,birthyear,country2) VALUES
	 ('da701e06-e025-4196-806f-38921a02b36a','Dávila Bermúdez','María Camila',NULL,'CO',NULL,NULL),
	 ('anonymous','Anónimo',NULL,'Anónimo',NULL,NULL,NULL),
	 ('ec2c3c2d-bc93-498a-8976-9fee8b585d8c','Cyrulnik','Boris',NULL,'FR',1937,NULL),
	 ('4d67ff8b-5ffc-4fbf-8ed2-142a53c5bf96','Sanín Paz','Carolina',NULL,'CO',1973,NULL),
	 ('6f66e7c8-7b6c-480f-9d2c-d311ef236dbb','Millán','José Antonio',NULL,'ES',1954,NULL),
	 ('e572d262-e0ee-45de-b3b6-4cac45ddf1f1','Peal','Robert',NULL,'GB',NULL,NULL),
	 ('3c16d402-1f3c-40d2-9e54-0cc2e8d743dd','Atkinson','James',NULL,'GB-ENG',1914,NULL),
	 ('bb05284e-d760-430b-832d-5b88b3f05185','Zweig','Stefan',NULL,'AT',NULL,'GB-ENG'),
	 ('e6465805-3d6d-4ba8-85be-5698283c5864','Rattner','Josef',NULL,'AT',1928,NULL),
	 ('3b5328c5-e95e-484e-aa81-e82e3b42902b',NULL,NULL,'Epicteto','GR',50,NULL);
INSERT INTO public.authors (id,last_name,first_name,pseudonym,country1,birthyear,country2) VALUES
	 ('20a979c7-e0b9-4311-b90f-627b79fca0ae','Camps','Victoria',NULL,'ES',1941,NULL),
	 ('e948a9f4-aad6-479e-ae2c-032c037f42b5','Ortega y Gasset','José',NULL,'ES',1883,NULL),
	 ('383047a9-e168-4308-83ca-8b501a30911a','Mair','Lucy',NULL,'GB-ENG',1901,NULL),
	 ('ebc82d73-88a3-418e-a78c-907d84679788','Lemaitre Ripoll','Julieta',NULL,'CO',1969,NULL),
	 ('2baf5ae4-db77-4457-a85c-40af897f706a','Gidard','René',NULL,'FR',1923,NULL),
	 ('8547b72a-adeb-44fa-8789-f80666ddea5b','Cees','Nooteboom',NULL,'NL',1933,NULL),
	 ('7b569f3e-c7cd-4469-9bcd-c76681ede497','Leibniz','Gottfried Wilhelm',NULL,'DE-SAC',1646,NULL),
	 ('e1e0ed14-df79-43e7-aba7-1249c8c0d77d','Eliot','Thomas Stearns','T. S. Eliot','US',1888,'GB-ENG'),
	 ('c38b1aa8-904f-4a06-a864-151fb82cf7c8','Sierra Mejía','Rubén',NULL,'CO',1937,NULL),
	 ('fb3242ce-b1bd-4e05-8cbf-31fa1b38c8a2','Parkinson','George Henry Radcliffe','G. H. R. Parkinson','GB-ENG',1923,NULL);
INSERT INTO public.authors (id,last_name,first_name,pseudonym,country1,birthyear,country2) VALUES
	 ('ecbdd718-5f23-4ee1-96bc-a8c44ddae7d4','Galeano','Eduardo',NULL,'UY',1940,NULL),
	 ('aec0286e-39c3-445b-bb13-e511300a7c28','Huberman','Edward',NULL,'GB-ENG',NULL,NULL),
	 ('4ed56932-5bee-47f1-9844-0d12fef1e92b','Huberman','Elizabeth',NULL,'GB-ENG',NULL,NULL);

INSERT INTO public.countries (iso3166,"name") VALUES
	 ('CO','Colombia'),
	 ('CA','Canadá'),
	 ('GB-SCT','Escocia'),
	 ('GB-ENG','Inglaterra'),
	 ('FR','Francia'),
	 ('US','Estados Unidos'),
	 ('DE','Alemania'),
	 ('ES','España'),
	 ('RU','Rusia'),
	 ('GB','Reino Unido');
INSERT INTO public.countries (iso3166,"name") VALUES
	 ('AT','Austria'),
	 ('GR','Grecia'),
	 ('NL','Países Bajos'),
	 ('DE-SAC','Sajonia'),
	 ('MX','México'),
	 ('UY','Uruguay'),
	 ('IE','Irlanda');

INSERT INTO public.languages (iso693_3,"name") VALUES
	 ('spa','Español'),
	 ('ang','Old English (ca.450–1100)'),
	 ('enm','Middle English (1100–1500)'),
	 ('eng','English'),
	 ('fra','Francés'),
	 ('deu','Alemán'),
	 ('jpn','Japonés'),
	 ('kbh','Camsá');