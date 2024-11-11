# settings.py - initializing the global variable

def init():

  global livefd;
  livefd = {};

  global closedfd;
  closedfd = [];

  global clonedfd;
  clonedfd = {};

  global logging;
  logging = False;

  global debugging;
  debugging = False;

  global elkserver;
  elkserver = "10.1.16.140:9200";

  global csvfile;
  csvfile = None;

  global jsonfile;
  jsonfile = None;

  global numdocs
  numdocs = 10000;

  global bulkdata
  bulkdata = [];

  global iddoc;
  iddoc = 1;

  global esindx;
  esindx = "";

  global dest;
  dest = "";
