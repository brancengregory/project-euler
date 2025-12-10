setwd("~/Documents/project-euler/Problem 54/")

library(tidyverse)
library(progress)

data <- read_table2("p054_poker.txt", col_names = FALSE)

getHand <- function(player, round, data) {
  if (player == 1) {
    data %>%
      slice(round) %>%
      unlist(., use.names = F) %>%
      .[1:5] %>%
      return()
  } else if(player == 2) {
    data %>%
      slice(round) %>%
      unlist(., use.names = F) %>%
      .[6:10] %>%
      return()
  } else {
    print("You can only select player 1 or 2")
    return(NULL)
  }
}

getValue <- function(card) {
  cards <- c(2, 3, 4, 5, 6, 7, 8, 9,
             "T", "J", "Q", "K", "A")
  values <- c(2, 3, 4, 5, 6, 7, 8, 9,
              10, 11, 12, 13, 14)
  valuesTable <- cbind(cards, values)
  card <- card %>%
    str_split(pattern = "") %>%
    unlist %>%
    .[1]
  which(valuesTable[,1] == card) %>%
    valuesTable[.,2] %>%
    unname() %>%
    as.numeric() %>%
    return
}

getSuit <- function(card) {
  card %>%
    str_split(pattern = "") %>%
    unlist %>%
    .[2]
}

orderHand <- function(hand) {
  hand <- sapply(hand, function(x){getValue(x)}) %>%
    sort()
  return(names(hand))
}

getHighCard <- function(hand) {
  hand %>%
    orderHand() %>%
    .[5] %>%
    return
}

hasRoyalFlush <- function(hand) {
  cond <- hand %>%
    hasStraightFlush()
  if (cond == T) {
    cond <- getHighCard(hand) %>%
      getValue()
    if (cond == 13) {
      return(T)
    } else {
      return(F)
    }
  }
  return(F)
}

hasStraightFlush <- function(hand) {
  if(hasFlush(hand)==T && hasStraight(hand)==T ) {
    return(T)
  }
  return(F)
}

hasFourofKind <- function(hand) {
  hand <- sapply(hand, function(x){getSuit(x)})
  num <- which(hand == hand[1]) %>%
    length()
  if (num == 4) {
    return(T)
  } else if (num > 4) {
    return(F)
  } else if (num < 4) {
    num <- which(hand == hand[2])
    if (num == 4) {
      return(T)
    } else {
      return(F)
    }
  }
  return(F)
}

hasFullHouse <- function(hand) {
  hand <- sapply(hand, function(x){getValue(x)})
  num <- which(hand == hand[1]) %>%
    length()
  if (num == 2) {
    hand <- hand[-which(hand == hand[1])]
    num <- which(hand == hand[1]) %>%
      length()
    if (num == 3) {
      return(T)
    } else {
      return(F)
    }
  } else if (num == 3) {
    hand <- hand[-which(hand == hand[1])]
    num <- which(hand == hand[1]) %>%
      length()
    if (num == 2) {
      return(T)
    } else {
      return(F)
    }
  }
  return(F)
}

hasFlush <- function(hand) {
  hand <- sapply(hand, function(x){getSuit(x)})
  num <- which(hand == hand[1]) %>%
    length
  if (num == 5) {
    return(T)
  }
  return(F)
}

isConsecutive <- function(values) {
  for(i in 1:(length(values)-1)) {
    if((values[i]+1) != (values[i+1])){
      return(F)
    } else {
      next()
    }
  }
  return(T)
}

hasStraight <- function(hand) {
  sapply(hand, function(x){getValue(x)}) %>%
    unname() %>%
    sort() %>%
    isConsecutive() %>%
    return
}

hasThreeofKind <- function(hand) {
  hand <- sapply(hand, function(x){getSuit(x)})
  num <- which(hand == hand[1]) %>%
    length()
  if (num == 3) {
    return(T)
  } else {
    num <- which(hand == hand[2]) %>%
      length()
    if (num == 3) {
      return(T)
    } else {
      num <- which(hand == hand[3]) %>%
        length()
      if (num == 3) {
        return(T)
      } else {
        return(F)
      }
    }
  }
  return(F)
}

hasTwoPair <- function(hand) {
  hand <- sapply(hand, function(x){getValue(x)})
  num <- which(hand == hand[1]) %>%
    length()
  if (num >= 2) {
    hand <- hand[-which(hand == hand[1])]
    if (length(hand) < 2) {
      return(F)
    } else if (length(hand) == 2) {
      if (hand[1] == hand[2]) {
        return(T)
      } else {
        return(F)
      }
    } else {
      num <- which(hand == hand[1]) %>%
        length()
      if (num >= 2) {
        return(T)
      } else {
        num <- which(hand == hand[2]) %>%
          length()
        if (num == 2) {
          return(T)
        } else {
          return(F)
        }
      }
    }
  } else {
    num <- which(hand == hand[2]) %>%
      length()
    if (num == 2) {
      hand <- hand[-1]
      hand <- hand[-which(hand == hand[1])]
      if (hand[1] == hand[2]) {
        return(T)
      } else {
        return(F)
      }
    } else {
      return(F)
    }
  }
  return(F)
}

hasOnePair <- function(hand) {
  hand <- sapply(hand, function(x){getValue(x)})
  num <- which(hand == hand[1]) %>%
    length()
  if (num == 2) {
    return(T)
  } else {
    num <- which(hand == hand[2]) %>%
      length()
    if(num == 2) {
      return(T)
    } else {
      num <- which(hand == hand[3]) %>%
        length()
      if (num == 2) {
        return(T)
      } else {
        num <- which(hand == hand[4]) %>%
          length()
        if(num == 2) {
          return(T)
        } else {
          return(F)
        }
      }
    }
  }
  return(F)
}

getRank <- function(player, round, data) {
  hand <- getHand(player, round, data)
  if (hasRoyalFlush(hand) == T) {
    return(10)
  } else if(hasStraightFlush(hand) == T) {
    return(9)
  } else if(hasFourofKind(hand) == T) {
    return(8)
  } else if(hasFullHouse(hand) == T) {
    return(7)
  } else if(hasFlush(hand) == T) {
    return(6)
  } else if(hasStraight(hand) == T) {
    return(5)
  } else if(hasThreeofKind(hand) == T) {
    return(4)
  } else if(hasTwoPair(hand) == T) {
    return(3)
  } else if(hasOnePair(hand) == T) {
    return(2)
  } else {
    return(1)
  }
}

whoWins <- function(round, data) {
  hand1 <- getHand(1, round, data)
  hand2 <- getHand(2, round, data)
  
  rank1 <- getRank(1, round, data)
  rank2 <- getRank(2, round, data)
  
  if (rank1 > rank2) {
    return(1)
  } else if (rank1 < rank2) {
    return(2)
  } else if (rank1 == rank2) {
    if (rank1 == 1) {
      if (getValue(getHighCard(hand1)) > getValue(getHighCard(hand2))) {
        return(1)
      } else {
        return(2)
      }
    } else if (rank1 == 2) {
      pairValue1 <- getValue(hand1[which(duplicated(hand1))])
      pairValue2 <- getValue(hand2[which(duplicated(hand2))])
      if (pairValue1 > pairValue2) {
        return(1)
      } else if (pairValue1 < pairValue2) {
        return(2)
      } else if (pairValue1 == pairValue2) {
        high1 <- getHighCard(hand1[-which(duplicated(hand1) | duplicated(hand1, fromLast = T))])
        high2 <- getHighCard(hand2[-which(duplicated(hand2) | duplicated(hand2, fromLast = T))])
        if (high1 > high2) {
          return(1)
        } else if (high1 < high2) {
          return(2)
        }
      }
    } else if (rank1 == 3) {
      
    } else if (rank1 == 4) {
      
    } else if (rank1 == 5) {
      
    } else if (rank1 == 6) {
      
    } else if (rank1 == 7) {
      
    } else if (rank1 == 8) {
      
    } else if (rank1 == 9) {
      
    } else if (rank1 == 10) {
      
    }
  }
}

winCount <- function(data) {
  progress <- progress_bar$new(total = nrow(data))
  table <- c()
  for(i in 1:nrow(data)) {
    progress$tick()
    table[i] <- whoWins(i, data)
  }
  return(table)
}

winCount(data)