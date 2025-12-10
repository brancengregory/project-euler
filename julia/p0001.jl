function isMultiple(x::Int64, y::Int64)::Bool
	if x % y == 0
		return true
	end
	return false
end

sum = 0
for i = 1:999
	if isMultiple(i, 3) || isMultiple(i, 5)
		global sum += i
	end
end

println(sum)