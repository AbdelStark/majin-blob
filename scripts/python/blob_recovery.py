# This script is a courtesy of @ArielElp
# It illustrates how to recover Starknet state diffs from an EIP-4844 blob.
# The script is not optimized for speed, and is intended for educational purposes only.


P = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
# z is the generator of the group of evaluation points (EIP-4844 parameter)
z = 39033254847818212395286706435128746857159659164139250548781411570340225835782
BLOB_LEN = 4096

def div_mod(a, b, P):
    return (a * pow(b, P - 2, P)) % P

def ifft(arr, xs):
    if len(arr) == 1:
        return arr
    n = len(arr) // 2
    res0 = []
    res1 = []
    new_xs = []
    for i in range(0, 2 * n, 2):
        a = arr[i]
        b = arr[i + 1]
        x = xs[i]
        res0.append(div_mod(a + b, 2, P))
        res1.append(div_mod(a - b, 2 * x, P))
        new_xs.append(pow(x, 2, P))
    return sum(zip(ifft(res0, new_xs), ifft(res1, new_xs)), ())

# Blob data taken from this blob on Goerli:
# https://goerli.blobscan.com/blob/0x01394306d3a5e6456771c2a4689e98269d220636723a877e44f19e11f6e57e6d
# Read blob_hex from a file
with open('./examples/blob/sn_blob_goerli.txt', 'r') as file:
    blob_hex = file.read().strip()

data = [int(blob_hex[i:i+64], 16) for i in range(0, BLOB_LEN * 64, 64)]
xs = [pow(z, int(bin(i)[2:].rjust(12, '0')[::-1], 2), P) for i in range(BLOB_LEN)]
res = ifft(data, xs)
print(res)