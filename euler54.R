setwd("~/Documents/project-euler/Problem 54/")

library(tidyverse)

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

getHighCard <- function(player, round, data) {
  getHand(player, round, data) %>%
    orderHand() %>%
    .[5] %>%
    return
}

hasRoyalFlush <- function(hand) {

}

hasStraightFlush <- function(x) {
  if(hasFlush(x)==T && hasStraight(x)==T ) {
    return(T)
  } else {
    return(F)
  }
}

hasFourofKind <- function() {
  
}

hasFullHouse <- function() {
  
}

hasFlush <- function(hand) {
  hand <- sapply(hand, function(x){getSuit(x)})
  num <- which(hand == hand[1]) %>%
    length
  if (num != 5) {
    return(F)
  } else {
    return(T)
  }
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

hasThreeofKind <- function() {
  
}

hasTwoPair <- function() {
  
}

hasOnePair <- function() {
  
}

whoWins <- function(round, data) {
  
}

whoWinsMost <- function(data) {
  
}