def minimalMultiple(divisor, lowerBound):
    return (lowerBound/divisor + (1 if lowerBound%divisor else 0))*divisor
