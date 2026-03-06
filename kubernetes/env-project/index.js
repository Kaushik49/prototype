const express = require('fs'); 

const path = require('path');

//ENV variables
// the database_url gets captured during the process, and cached in the variable, it needs to wait to get this variable
const DATABASE_URL = process.env.DATABASE_URL; 
if(!DATABASE_URL){
    console.error('please inject database url')
}

function generateRandomData(length){
    let characters = 'dsfasadsfaerhtgjddsftrwe'
    let results = '';
    for(let i=0 ;i<length;i++){
        results = characters.charAt(Math.floor(Math.random()*characters.length))
    }
    return results
}