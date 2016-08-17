-- Super correct
import Data.List.Split

sortAndCount :: [Int] -> ([Int], Int)
sortAndCount (a:[]) = ([a], 0)
sortAndCount a = (reverse d, x + y + z)
  where
    n = length a
    (l1, l2) = ((take (n `div` 2) a), (drop (n `div` 2) a))
    (b, x) = sortAndCount l1
    (c, y) = sortAndCount l2
    (d, z) = mergeAndCountSplit b c

mergeAndCountSplit :: [Int] -> [Int] -> ([Int], Int)
mergeAndCountSplit a b = mergeAndCountSplit' a b [] 0

-- first, second, accum, countAccum
mergeAndCountSplit' :: [Int] -> [Int] -> [Int] -> Int -> ([Int], Int)
mergeAndCountSplit' as [] accum count = ((reverse as) ++ accum, count)
mergeAndCountSplit' [] bs accum count = ((reverse bs) ++ accum, count)
mergeAndCountSplit' (a:as) (b:bs) accum count
  | a > b = mergeAndCountSplit' (a:as) bs (b:accum) (count + (length (a:as)))
  | otherwise = mergeAndCountSplit' as (b:bs) (a:accum) count

mergesort :: [Int] -> [Int]
mergesort [] = []
mergesort (a:[]) = [a]
mergesort as = merge (mergesort l1) (mergesort l2)
  where
  (l1, l2) = splitAt (div (length as) 2) as

merge :: [Int] -> [Int] -> [Int]
merge l1 l2 = reverse $ merge' l1 l2 []

merge' :: [Int] -> [Int] -> [Int] -> [Int]
merge' as [] accum = (reverse as) ++ accum
merge' [] bs accum = (reverse bs) ++ accum
merge' (a:as) (b:bs) accum
  | a > b = merge' (a:as) bs (b:accum)
  | otherwise = merge' as (b:bs) (a:accum)

run = do
  content <- readFile "integer_array.txt"
  let integers = (map read $ lines content :: [Int])
  putStrLn $ show $ length integers
  let (_, c) = sortAndCount integers
  putStrLn $ show c

