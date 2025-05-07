local function fib(n)
    if n == 0 or n == 1 then
        return n
    else
        return fib(n - 1) + fib(n - 2)
    end
end


local n = io.read("*n")
print(fib(n))
