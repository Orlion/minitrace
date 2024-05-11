<?php
$host = '127.0.0.1';
$db   = 'blog';
$user = 'root';
$pass = '123456';
$charset = 'utf8mb4';
$dsn = "mysql:host=$host;dbname=$db;charset=$charset";
$options = [
    PDO::ATTR_ERRMODE            => PDO::ERRMODE_EXCEPTION,
    PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
    PDO::ATTR_EMULATE_PREPARES   => false,
];
$pdo = new PDO($dsn, $user, $pass, $options);
$stm = $pdo->query('select * from article');
$rows = $stm->fetchAll();
foreach($rows as $row) {
    print_r($row);
}
