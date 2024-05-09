<?php

$ch = curl_init();
curl_setopt($ch, CURLOPT_URL, 'https://error.blog.fanscore.cn/a/57/?k1=v1&k2=k2&k3=v3#aaa');
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
$response = curl_exec($ch);
curl_close($ch);