main = do
    contents <- readFile "../../inputs/day_01.txt"
    putStrLn contents

x contents = lines contents

offsets :: [String] -> [Int]
offsets line = [ if x == 'R' then (read xs :: Int)  else -(read xs :: Int) | x:xs <- line ]
