from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.bool import TRUE, FALSE
from starkware.cairo.common.memcpy import memcpy
from starkware.cairo.common.memset import memset 

# Helper functions
func fill_array(array : felt*, base : felt, step : felt, array_length : felt, iterator : felt):
    if iterator == array_length:
        return()
    end
    assert array[iterator] = base + step * iterator
    return fill_array(array, base, step, array_length, iterator + 1)
end

func check_array(array : felt*, value : felt, array_length : felt, iterator : felt ) -> (r : felt):
    if iterator == array_length:
        return(TRUE)
    end
    if array[iterator] != value:
        return(FALSE)
    end
    return check_array(array, value, array_length, iterator + 1)
end

func compare_arrays(array_a : felt*, array_b : felt*, array_length : felt, iterator : felt ) -> (r : felt):
    if iterator == array_length:
        return(TRUE)
    end
    if array_a[iterator] != array_b[iterator]:
        return(FALSE)
    end
    return compare_arrays(array_a, array_b, array_length, iterator + 1)
end

# ---------------------------------------------------------------------------------- #

# Test functions
func test_memcpy(src : felt*, len : felt, iter : felt) -> ():
    alloc_locals
    if iter == len:
        return ()
    end

    let (dst : felt*) = alloc()
    memcpy(dst=dst, src=src, len=len)

    let result : felt = compare_arrays(src, dst, len, 0)
    assert result = TRUE

    return test_memcpy(src, len, iter + 1)
end

func test_memset(n : felt, iter : felt) -> ():
    alloc_locals
    if iter == n:
        return ()
    end

    let (dst : felt*) = alloc()
    memset(dst=dst, value=1234, n=n)

    let result : felt =  check_array(dst, 1234, n, 0)
    assert result = TRUE

    return test_memset(n, iter + 1)
end

func test_integration(src : felt*, len : felt, n : felt, iter : felt) -> ():
    alloc_locals
    if iter == len:
        return ()
    end

    let (initial_dst : felt*) = alloc()
    memcpy(dst=initial_dst, src=src, len=len)
    let res_1 : felt = compare_arrays(src, initial_dst, len, 0)
    assert res_1 = TRUE

    let (dst : felt*) = alloc()
    memset(dst=dst, value=initial_dst[iter], n=n)
    let res_2 : felt =  check_array(dst, initial_dst[iter], n, 0)
    assert res_2 = TRUE

    let (final_dst : felt*) = alloc()
    memcpy(dst=final_dst, src=dst, len=n)
    let res_3 : felt = check_array(final_dst, initial_dst[iter], n, 0)
    assert res_3 = TRUE

    return test_integration(src, len, n, iter + 1)
end

func run_tests(len : felt, n : felt) -> ():
    alloc_locals
    
    let (array : felt*) = alloc()
    fill_array(array, 7, 3, len, 0)

    test_memcpy(array, len, 0) 
    test_memset(n, 0)
    test_integration(array, len, n, 0)

    return ()
end

func main():
    run_tests(100, 50)
    return ()
end
