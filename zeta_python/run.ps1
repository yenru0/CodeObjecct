if ($args.Length -gt 0) {
    type stdin.txt | python $args[0]
} else {
    Write-Host "No args"
}
