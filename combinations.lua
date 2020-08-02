num_of_fingers = 4
num_of_keys = 7

keys="0123456789abcde"

function str_to_set(str)
   local tab = {}
   tab.len = 0

   for i=1, #str do
      if not tab[sub(str, i, i)] then
         tab[sub(str, i, i)] = true
         tab.len += 1
      end
   end

   return tab
end

function set_equals(set1, set2)
   if set1.len != set2.len then
      return false
   end

   for k, v in pairs(set1) do
      if not set2[k] then
         return false
      end
   end

   for k, v in pairs(set2) do
      if not set1[k] then
         return false
      end
   end

   return true
end

function set_to_str(set)
   local cur_str = ""
   for k, v in pairs(set) do
      if k != "len" then
         cur_str = cur_str..k
      end
   end
   return cur_str
end

function does_list_contain_set(list, set)
   for i=1, #list do
      if set_equals(list[i], set) then
         return true
      end
   end

   return false
end

all_combos = {}
function calc(str, cur)
   local number = 0
   for i=cur, num_of_keys do
      local cur_str = str..sub(keys, i, i)
      local cur_set = str_to_set(cur_str)

      if not does_list_contain_set(all_combos, cur_set) then
         number += 1
         add(all_combos, cur_set)
      end
      if #cur_str < num_of_fingers and i < num_of_keys then
         number += calc(cur_str, i+1)
      end
   end

   return number
end

-- printh(set_equals({1}, {1, 2}))
total = calc("", 1)

for x in all(all_combos) do
   printh(set_to_str(x))
end

printh("total: "..total)
