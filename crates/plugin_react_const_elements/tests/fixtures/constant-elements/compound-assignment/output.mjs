let _Loader, _Loader2;
import React from 'react';
import Loader from 'loader';
const errorComesHere = ()=>{
  return _Loader || (_Loader = <Loader className="full-height"/>);
}, thisWorksFine = ()=>{
  return _Loader2 || (_Loader2 = <Loader className="p-y-5"/>);
};
