<?php

class Abc {

}

$ch = curl_init();
curl_setopt($ch, CURLOPT_URL, 'https://error.blog.fanscore.cn/a/57/?k1=v1&k2=k2&k3=v3#aaa');
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
$response = curl_exec($ch);
curl_close($ch);
var_dump(false);
var_dump(true);
var_dump(1);
var_dump("123");
var_dump([1,2,3]);
var_dump(new Abc());