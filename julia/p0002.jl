function isEven(x::Int64)
	if x % 2 == 0
		return true
	end
	return false
end

function getFibs(x::Int64)
	fib = [1, 2]
	while fib[end] < x 
		push!(fib, fib[end]+fib[end-1])
	end
	if fib[end] > x
		pop!(fib)
	end
	return fib
end

function getSumOfEvens(x)
	sum = 0
	for n in x
		if isEven(n)
			sum += n
		end
	end
	return sum
end

fibs = getFibs(4000000)
println(getSumOfEvens(fibs))
