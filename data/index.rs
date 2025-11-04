%% glyphs_cmssbx10_bottom_save.ldf
%% Created by Laurence D. Finston (LDF) Sa 4. Okt 10:34:07 CEST 2025

%% * (0) Copyright and License.

%% This file is part of GNU 3DLDF, a package for three-dimensional drawing. 
%% Copyright (C) 2025 The Free Software Foundation, Inc. 

%% GNU 3DLDF is free software; you can redistribute it and/or modify 
%% it under the terms of the GNU General Public License as published by 
%% the Free Software Foundation; either version 3 of the License, or 
%% (at your option) any later version. 

%% GNU 3DLDF is distributed in the hope that it will be useful, 
%% but WITHOUT ANY WARRANTY; without even the implied warranty of 
%% MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the 
%% GNU General Public License for more details. 

%% You should have received a copy of the GNU General Public License 
%% along with GNU 3DLDF; if not, write to the Free Software 
%% Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA 

%% GNU 3DLDF is a GNU package.  
%% It is part of the GNU Project of the  
%% Free Software Foundation 
%% and is published under the GNU General Public License. 
%% See the website http://www.gnu.org 
%% for more information.   
%% GNU 3DLDF is available for downloading from 
%% http://www.gnu.org/software/3dldf/LDF.html. 

%% ("@@" stands for a single at-sign in the following paragraph.) 

%% Please send bug reports to Laurence.Finston@@gmx.de 
%% The mailing list help-3dldf@@gnu.org is available for people to 
%% ask other users for help.  
%% The mailing list info-3dldf@@gnu.org is for sending 
%% announcements to users. To subscribe to these mailing lists, send an 
%% email with "subscribe <email-address>" as the subject.  

%% The author can be contacted at: 

%% Laurence D. Finston                  
%% c/o Free Software Foundation, Inc.  
%% 51 Franklin St, Fifth Floor          
%% Boston, MA  02110-1301              
%% USA                                 


%% ** (2) ' (Apostrophe) (Special characters 3)

  glyph Apostrophe;
  Apostrophe := get_glyph 39 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Apostrophe;

  scale qv (10, 10);

  Q33 := qv0;
  Q34 := qv1;


  p389 := p380 shifted (6, 0);

  p390 := (xpart get_point 4 Q33, ypart get_point 0 Q34);

  
  shift Q33 by (p389 - p390);
  shift Q34 by (p389 - p390);

  p390 := get_point 1 Q34;
  p391 := get_point 2 Q34;
  p392 := (xpart p389, ypart p391);

  Q34 := p389 -- p390 -- p391 -- p392 -- cycle;
  
  draw Q34 with_color gray;
  draw Q33;
  
  if do_labels:
    dotlabel.bot("$p_{389}$", p389);
    dotlabel.bot("$p_{390}$", p390);
    dotlabel.top("$p_{391}$", p391);
    dotlabel.top("$p_{392}$", p392); 
  fi;
  
  if false: % true:
    n := (size Q33) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q33 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q33) with_color red;
    endfor;
  fi;

  if false: % true:
    n := (size Q34) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      dotlabel.bot(s, get_point (i) Q34) with_color dark_green;
    endfor;
  fi;

  





%% ** (2)  

endfig with_projection parallel_x_y;
fig_ctr += 1;

%% ** (2)

beginfig(fig_ctr);

%% ** (2)

  draw frame with_pen big_square_pen;
  
  if do_labels:
    dotlabel.bot("$p_{0}$", p0);
    dotlabel.bot("$p_{1}$", p1);
    dotlabel.top("$p_{2}$", p2);
    dotlabel.top("$p_{3}$", p3);

    dotlabel.lft("$p_{10}$", p10);
    dotlabel.rt("$p_{11}$", p11);
    dotlabel.lft("$p_{64}$", p64);
    dotlabel.rt("$p_{65}$", p65);
    
    dotlabel.lft("$p_{101}$", p101);
    dotlabel.rt("$p_{102}$", p102);
  fi;

 
%% ** (2) # (Hash mark (number sign))

  glyph Hash_mark;
  Hash_mark := get_glyph 35 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Hash_mark;

  scale qv (10, 10);

  Q35 := qv0;
  Q36 := qv1;
  Q37 := qv2;

  p399 := p10 shifted (2, -2);

  p400 := (xpart get_point 20 Q35, ypart get_point 24 Q35);

  shift Q35 by (p399 - p400);
  shift Q36 by (p399 - p400);
  shift Q37 by (p399 - p400);
  

  p400 := get_point 1 Q37;
  p401 := get_point 2 Q37;
  p402 := (xpart p399, ypart p401);
  p403 := mediate(p399, p401);

  Q37 := p399 -- p400 -- p401 -- p402 -- cycle;
  
  draw Q37 with_color gray;
  draw Q35;
  draw Q36;

  if do_labels:
    dotlabel.bot("$p_{399}$", p399);
    dotlabel.bot("$p_{400}$", p400);
    dotlabel.top("$p_{401}$", p401);
    dotlabel.top("$p_{402}$", p402);
    dotlabel.bot("$p_{403}$", p403);
    label.ulft("$Q_{35}$", get_point 13 Q35);
    label.top("$Q_{36}$", p403);
    label.lrt("$Q_{37}$", p402) with_color dark_gray;
  fi;

  
  if false: % true:
    n := (size Q35) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q35 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q35) with_color red;
    endfor;
  fi;

  
%% ** (2) # (Hash mark (number sign))

  glyph At_sign;
  At_sign := get_glyph 64 from "cmssbx10";

  clear qv;
  
  qv := get_paths from At_sign;

  scale qv (10, 10);

  Q38 := qv0;
  Q39 := qv1;
  Q40 := qv2;

  p408 := p400 shifted (4, 0);
  p409 := (xpart get_point 13 Q38, ypart get_point 14 Q38);
  
  shift Q38 by (p408 - p409);
  shift Q39 by (p408 - p409);
  shift Q40 by (p408 - p409);

  fill Q38 with_color gray;
  unfill Q39;
  

  p409 := get_point 1 Q40;
  p410 := get_point 2 Q40;
  p411 := (xpart p408, ypart p410);
  p412 := mediate(p408, p410);

  Q40 := p408 -- p409 -- p410 -- p411 -- cycle;

  draw Q40 with_color gray;
  draw Q38;
  draw Q39;

  if do_labels:
    dotlabel.bot("$p_{408}$", p408);
    dotlabel.bot("$p_{409}$", p409);
    dotlabel.top("$p_{410}$", p410);
    dotlabel.top("$p_{411}$", p411);
    dotlabel.bot("$p_{412}$", p412);
    label.bot("$Q_{39}$", p412 shifted (1.125, 1.5));
    label.top("$Q_{38}$", mediate(p411, p410));
  fi;



  if false: % true: % 
    n := (size Q38) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q38 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q38) with_color red;
    endfor;
  fi;




%% ** (2)
  
endfig with_projection parallel_x_y;
fig_ctr += 1;


%% ** (2)

% Quotation marks:  oct 042 == dec. 34, oct 0134 == dec. 92

%% ** (2)

numeric offset;
offset := 1.5;

%% * (1) Make larger surrounding boxes.

do_labels := true; % false; % 

%% ** (2) First figure with larger surrounding boxes.

beginfig(fig_ctr);

%% ** (3) A (q100).

  draw frame shifted (-4, 4) with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;
  
  draw q0;
  draw q1;
  %draw q2;

  
  p108 := get_point 2 q2;;

  if do_labels:
    dotlabel.top("$_{35}$", p35);
    dotlabel.top("$_{37}$", p37);
    dotlabel.top("$_{108}$", p108);

    dotlabel.bot("$_{34}$", p34);
    dotlabel.bot("$_{36}$", p36);
    dotlabel.bot("$_{38}$", p38);
    dotlabel.bot("$_{39}$", p39);
  fi;

  p109 := mediate(p36, p108);

  p110 := p36 shifted (-offset, -offset);

  p111 := p39 shifted (offset, -offset);

  p112 := p108 shifted (offset, offset);

  p113 := p37 shifted (-offset, offset);


  if do_labels:
    dotlabel.bot("$_{109}$", p109);
    dotlabel.bot("$_{110}$", p110);
    dotlabel.bot("$_{111}$", p111);  
    dotlabel.top("$_{112}$", p112);
    dotlabel.top("$_{113}$", p113);
  fi;

  q100 := p110 -- p111 -- p112 -- p113 -- cycle;

  p137 := mediate(p110, p111);
  
  if do_labels:
    dotlabel.bot("$p_{137}$", p137);
    
    draw q100 with_color red;
    label.top("$q_{100}$", mediate(p113, p112)) with_color red; 
  else:
    draw q100;
  fi;
  
  v1 := current_picture;
  clear current_picture;

%% ** (3) P (q103) (AP)

  draw frame shifted (-4, 4) with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

  draw q3;
  draw q4;
  %draw q5;

  p117 := get_point 2 q5;
  p118 := get_point 3 q5;

  p119 := p40 shifted (-offset, -offset);
  p120 := p42 shifted (offset, -offset);
  p121 := p117 shifted (offset, offset);
  p122 := p118 shifted (-offset, offset);

  q103 := p119 -- p120 -- p121 -- p122 -- cycle;
 
  p123 := mediate(p40, p117);

  p138 := mediate(p119, p120);
  
  if do_labels:
    dotlabel.bot("$p_{40}$", p40);
    dotlabel.bot("$p_{42}$", p42);
    dotlabel.bot("$p_{43}$", p43);
    dotlabel.top("$p_{117}$", p117);
    dotlabel.top("$p_{118}$", p118);
    dotlabel.bot("$p_{119}$", p119);
    dotlabel.bot("$p_{120}$", p120);
    dotlabel.top("$p_{121}$", p121);
    dotlabel.top("$p_{122}$", p122);
    dotlabel.bot("$p_{123}$", p123);
    dotlabel.bot("$p_{138}$", p138);
    draw q103 with_color red;
    label.top("$q_{103}$", mediate(p122, p121)) with_color red;
  else:
    draw q103;
  fi;

  v2 := current_picture;
  clear current_picture;

%% ** (3) R (q106) (A PR)

  draw frame shifted (-4, 4) with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

  draw q6;
  draw q7;
  %draw q8;

  p124 := get_point 2 q8;

  p125 := mediate(p44, p124);

  p126 := p44 shifted (-offset, -offset);
  p127 := p47 shifted (offset, -offset);
  p128 := p124 shifted (offset, offset);
  p129 := p46 shifted (-offset, offset);

  p139 := mediate(p126, p127);
  
  q106 := p126 -- p127 -- p128 -- p129 -- cycle;
  
  if do_labels:
    dotlabel.bot("$p_{44}$", p44);
    dotlabel.top("$p_{46}$", p46);
    dotlabel.bot("$p_{47}$", p47);
    dotlabel.bot("$p_{48}$", p48);
    dotlabel.top("$p_{124}$", p124);
    dotlabel.bot("$p_{125}$", p125);
    dotlabel.bot("$p_{126}$", p126);
    dotlabel.bot("$p_{127}$", p127);
    dotlabel.top("$p_{128}$", p128);
    dotlabel.top("$p_{129}$", p129);
    dotlabel.bot("$p_{139}$", p139);
    label.top("$q_{106}$", mediate(p128, p129)) with_color red;
    draw q106 with_color red;
  else:
    draw q106;
  fi;

  v3 := current_picture;
  clear current_picture;
  
  % shift current_picture (6, 0);
  % output current_picture with_projection parallel_x_y;
  % clear current_picture;
  
%% ** (3) E (q109) (A PRE)

  draw frame shifted (-4, 4) with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

  draw q9;
  %draw q10;

  p130 := get_point 2 q10;
  p131 := get_point 3 q10;

  p132 := mediate(p49, p130);

  p133 := p49 shifted (-offset, -offset);
  p134 := p51 shifted (offset, -offset);
  p135 := p130 shifted (offset, offset);
  p136 := p131 shifted (-offset, offset);
  p140 := mediate(p133, p134);

  
  q109 := p133 -- p134 -- p135 -- p136 -- cycle;

  
  
  if do_labels:
    dotlabel.bot("$p_{49}$", p49);
    dotlabel.bot("$p_{51}$", p51);
    dotlabel.bot("$p_{52}$", p52);
    dotlabel.top("$p_{130}$", p130);
    dotlabel.top("$p_{131}$", p131);
    dotlabel.bot("$p_{132}$", p132);
    dotlabel.bot("$p_{133}$", p133);
    dotlabel.bot("$p_{134}$", p134);
    dotlabel.top("$p_{135}$", p135);
    dotlabel.top("$p_{136}$", p136);
    dotlabel.bot("$p_{140}$", p140);
    label.top("$q_{109}$", mediate(p135, p136)) with_color red;
    draw q109 with_color red;
  else:
    draw q109;
  fi;

  v4 := current_picture;
  clear current_picture;
  
  % shift current_picture (8, 0);
  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

%% ** (3) T (q111) (A PRET)

  draw frame shifted (-4, 4) with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

  draw q11;
  %draw q12;

  p141 := get_point 2 q12;

  p142 := p54 shifted (-offset, -offset);
  p143 := p57 shifted (offset, -offset);
  p144 := p141 shifted (offset, offset);
  p145 := p56 shifted (-offset, offset);
  p146 := mediate(p142, p143);
  p147 := mediate(p54, p141);
  
  q111 := p142 -- p143 -- p144 -- p145 -- cycle;


  
  if do_labels:
    dotlabel.bot("$p_{54}$", p54);
    dotlabel.top("$p_{56}$", p56);
    dotlabel.bot("$p_{57}$", p57);
    dotlabel.bot("$p_{58}$", p58);
    dotlabel.top("$p_{141}$", p141);
    dotlabel.bot("$p_{142}$", p142);
    dotlabel.bot("$p_{143}$", p143);
    dotlabel.top("$p_{144}$", p144);
    dotlabel.top("$p_{145}$", p145);
    dotlabel.bot("$p_{146}$", p146);
    dotlabel.bot("$p_{147}$", p147);
    dotlabel.top("$q_{111}$", mediate(p144, p145)) with_color red;
    draw q111 with_color red;
  else:
    draw q111;
  fi;

  % shift current_picture (-38, -12);
  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

%% ** (3) Y (q115) (A PRETTY)

  draw q15;
  %draw q16;

  p398 := get_point 2 q16;
  p148 := mediate(p59, p398);
  p149 := p59 shifted (-offset, -offset);
  p150 := p61 shifted (offset, -offset);
  p151 := p398 shifted (offset, offset);
  p152 := p62 shifted (-offset, offset);
  p165 := mediate(p149, p150);

  
  q115 := p149 -- p150 -- p151 -- p152 -- cycle;

  
  if do_labels:
    dotlabel.bot("$p_{59}$", p59);
    dotlabel.bot("$p_{61}$", p61);
    dotlabel.top("$p_{62}$", p62);
    dotlabel.bot("$p_{63}$", p63);
    dotlabel.top("$p_{398}$", p398);
    dotlabel.bot("$p_{148}$", p148);
    dotlabel.bot("$p_{149}$", p149);
    dotlabel.bot("$p_{150}$", p150);
    dotlabel.top("$p_{151}$", p151);
    dotlabel.top("$p_{152}$", p152);
    dotlabel.bot("$p_{165}$", p165);
    label.top("$q_{115}$", mediate(p151, p152)) with_color red;
    draw q115 with_color red;
  else:
    draw q115;
  fi;

%% **** (4) G

  draw q17;
  %draw q18;

  p153 := get_point 2 q18;

  p154 := mediate(p66, p153);

  p155 := p66 shifted (-offset, -offset);
  p156 := p68 shifted (offset, -offset);
  p157 := p153 shifted (offset, offset);
  p158 := p69 shifted (-offset, offset);
  p166 := mediate(p155, p156);


  q117 := p155 -- p156 -- p157 -- p158 -- cycle;
  
  if do_labels:
    dotlabel.bot("$p_{66}$", p66);
    dotlabel.bot("$p_{68}$", p68);
    dotlabel.top("$p_{69}$", p69);
    dotlabel.bot("$p_{70}$", p70);
    dotlabel.top("$p_{153}$", p153);
    dotlabel.bot("$p_{154}$", p154);
    dotlabel.bot("$p_{155}$", p155);
    dotlabel.bot("$p_{156}$", p156);
    dotlabel.top("$p_{157}$", p157);
    dotlabel.top("$p_{158}$", p158);
    dotlabel.bot("$p_{166}$", p166);
    label.top("$q_{117}$", mediate(p157, p158)) with_color red;
    draw q117 with_color red;
  else:
    draw q117;
  fi;

%% **** (4) I

  draw q19;
  draw q20;

  p159 := get_point 2 q20;

  p160 := p71 shifted (-offset, -offset);
  p161 := p74 shifted (offset, -offset);
  p162 := p159 shifted (offset, offset);
  p163 := p73 shifted (-offset, offset);
  p164 := mediate(p160, p162);
  p167 := mediate(p160, p161);

  q119 := p160 -- p161 -- p162 -- p163 -- cycle;
  
  if do_labels:
    dotlabel.llft("$p_{71}$", p71);
    dotlabel.ulft("$p_{73}$", p73);
    dotlabel.lrt("$p_{74}$", p74);
    dotlabel.bot("$p_{75}$", p75);
    dotlabel.urt("$p_{159}$", p159);
    dotlabel.bot("$p_{160}$", p160);
    dotlabel.bot("$p_{161}$", p161);
    dotlabel.top("$p_{162}$", p162);
    dotlabel.top("$p_{163}$", p163);
    dotlabel.bot("$p_{164}$", p164);
    dotlabel.bot("$p_{167}$", p167);
    label.top("$q_{119}$", mediate(p162, p163)) with_color red;
    draw q119 with_color red;
  else:
    draw q119;
  fi;

%% **** (4)
    
%% ** (3) Combine the pictures.

%% **** (4) A (v1)

  temp_picture := v1;
  shift temp_picture (-3, 4);
  current_picture += temp_picture;

%% **** (4) P (v2) (A P)
  
  temp_picture := v2;
  shift temp_picture (2, 4);
  current_picture += temp_picture;

%% **** (4) R (v3) (A PR)
  
  temp_picture := v3;
  shift temp_picture (8, 4);
  current_picture += temp_picture;
  
%% **** (4) E (v4) (A PRE)

  temp_picture := v4;
  shift temp_picture (13, 4);
  current_picture += temp_picture;

%% **** (4) T (v5) (A PRETT)

  temp_picture := v5;
  shift temp_picture (-38, -12);
  current_picture += temp_picture;

%% **** (4) Y (v6) (A PRETTY)

  temp_picture := v6;
  shift temp_picture (-32, -12);
  current_picture += temp_picture;

%% **** (4) G (v7) (A PRETTY G)

  temp_picture := v7;
  shift temp_picture (30, 0);
  current_picture += temp_picture;  

%% **** (4) I (v8) (A PRETTY GI)
  
  temp_picture := v8;
  shift temp_picture (37, 0);
  current_picture += temp_picture;  

%% ** (3) End of first figure with larger surrounding boxes.
      
endfig with_projection parallel_x_y;
fig_ctr += 1;  

%% ** (2) Second figure with larger surrounding boxes.

beginfig(fig_ctr);

%% ** (2)  
  

%% ** (2) L (q124) (A PRETTY GIRL)

  draw q24;
  %draw q25;

  p168 := get_point 2 q25;

  p169 := p76 shifted (-offset, -offset);
  p170 := p79 shifted (offset, -offset);
  p171 := p168 shifted (offset, offset);
  p172 := p78 shifted (-offset, offset);
  p173 := mediate(p76, p168);
  p174 := mediate(p169, p170);
  
  q124 := p169 -- p170 -- p171 -- p172 -- cycle;
  
  if do_labels:
    dotlabel.bot("$p_{76}$", p76);
    dotlabel.top("$p_{78}$", p78);
    dotlabel.bot("$p_{79}$", p79);
    dotlabel.bot("$p_{80}$", p80);
    dotlabel.top("$p_{168}$", p168);
    dotlabel.bot("$p_{169}$", p169);
    dotlabel.bot("$p_{170}$", p170);
    dotlabel.top("$p_{171}$", p171);
    dotlabel.top("$p_{172}$", p172);
    dotlabel.bot("$p_{173}$", p173);
    dotlabel.bot("$p_{174}$", p174);
    label.top("$q_{124}$", mediate(p171, p172)) with_color red;
    draw q124 with_color red;
  else:
    draw q124;
  fi;

%% ** (2) S (q128) (A PRETTY GIRL)

  draw q28;
  %draw q29;

  p175 := get_point 2 q29;
  p176 := mediate(p81, p175);

  p177 := p81  shifted (-offset, -offset);
  p178 := p83  shifted (offset, -offset);
  p179 := p175 shifted (offset, offset);
  p180 := p84  shifted (-offset, offset);

  q128 := p177 -- p178 -- p179 -- p180 -- cycle;
  
  p181 := mediate(p177, p178);

  if do_labels:
    dotlabel.bot("$p_{81}$", p81);
    dotlabel.bot("$p_{83}$", p83);
    dotlabel.top("$p_{84}$", p84);
    dotlabel.bot("$p_{85}$", p85);
    dotlabel.top("$p_{175}$", p175);
    dotlabel.bot("$p_{176}$", p176);
    dotlabel.bot("$p_{177}$", p177);
    dotlabel.bot("$p_{178}$", p178);
    dotlabel.top("$p_{179}$", p179);
    dotlabel.top("$p_{180}$", p180);
    dotlabel.bot("$p_{181}$", p181);	
    dotlabel.top("$q_{128}$", mediate(p179, p180)) with_color red;
    draw q128 with_color red;
  else:
    draw q128;
  fi;

  v10 := current_picture;
  clear current_picture;

%% ** (2) K (q134) (v11) (A PRETTY GIRL IS LIK)
 
  draw q34;
  %draw q35;

  p182 := get_point 2 q35;

  p183 := p86 shifted (-offset, -offset);
  p184 := p88 shifted (offset, -offset);
  p185 := p182 shifted (offset, offset);
  p186 := p89 shifted (-offset, offset);
  p187 := mediate(p183, p184);
  p188 := mediate(p86, p182);


  q134 := p183 -- p184 -- p185 -- p186 -- cycle;
  
  if do_labels:
    dotlabel.bot("$p_{86}$", p86);
    dotlabel.bot("$p_{88}$", p88);
    dotlabel.top("$p_{89}$", p89);
    dotlabel.bot("$p_{90}$", p90);
    dotlabel.top("$p_{182}$", p182);
    dotlabel.bot("$p_{183}$", p183);
    dotlabel.bot("$p_{184}$", p184);
    dotlabel.top("$p_{185}$", p185);
    dotlabel.top("$p_{186}$", p186);
    dotlabel.bot("$p_{187}$", p187);
    dotlabel.bot("$p_{188}$", p188);
    label.top("$q_{134}$", mediate(p186, p185)) with_color red;
    draw q134 with_color red;
    else:
      draw q134;
  fi;  

  v11 := current_picture;
  clear current_picture;
  
%% ** (2) M (q141) (v12) (A PRETTY GIRL IS LIKE A M)

  draw q41;
  %draw q42;

  p189 := get_point 2 q42;

  p190 := p91 shifted (-offset, -offset);
  p191 := p93 shifted (offset, -offset);
  p192 := p189 shifted (offset, offset);
  p193 := p94 shifted (-offset, offset);
  p194 := mediate(p190, p191);
  p195 := mediate(p190, p192);

  q141 := p190 -- p191 -- p192 -- p193 -- cycle;
  
  if do_labels:
    dotlabel.bot("$p_{91}$", p91);
    dotlabel.bot("$p_{93}$", p93);
    dotlabel.top("$p_{94}$", p94);
    dotlabel.bot("$p_{95}$", p95);
    dotlabel.top("$p_{189}$", p189);
    dotlabel.bot("$p_{190}$", p190);
    dotlabel.bot("$p_{191}$", p191);
    dotlabel.top("$p_{192}$", p192);
    dotlabel.top("$p_{193}$", p193);
    dotlabel.bot("$p_{194}$", p194);
    dotlabel.bot("$p_{195}$", p195);
    label.top("$q_{141}$", mediate(p192, p193)) with_color red;
    draw q141 with_color red;
  else:
    draw q141;
  fi;

  
  
  % shift current_picture (3, 14);
  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

  v12 := current_picture;
  clear current_picture;

%% ** (2) O (q147) (v13) (A PRETTY GIRL IS LIKE A MELO)

  draw q47;
  draw q48;
  %draw q49;

  p196 := get_point 2 q49;

  p197 := mediate(p96, p196);

  p198 := p96 shifted (-offset, -offset);
  p199 := p98 shifted (offset, -offset);
  p200 := p196 shifted (offset, offset);
  p201 := p99 shifted (-offset, offset);
  p202 := mediate(p198, p199);
  
  q147 := p198 -- p199 -- p200 -- p201 -- cycle;
  
  if do_labels:
    dotlabel.bot("$p_{96}$", p96);
    dotlabel.bot("$p_{98}$", p98);
    dotlabel.top("$p_{99}$", p99);
    dotlabel.bot("$p_{100}$", p100);
    dotlabel.top("$p_{196}$", p196);
    dotlabel.bot("$p_{197}$", p197);
    dotlabel.bot("$p_{198}$", p198);
    dotlabel.bot("$p_{199}$", p199);
    dotlabel.top("$p_{200}$", p200);
    dotlabel.top("$p_{201}$", p201);
    dotlabel.bot("$p_{202}$", p202);
    label.top("$q_{147}$", mediate(p201, p200)) with_color red;
    draw q147 with_color red;
  else:
    draw q147;
  fi;

  % shift current_picture (-52, -2);
  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

  v13 := current_picture;
  clear current_picture;
  
%% ** (2) D (q150) (v14) (A PRETTY GIRL IS LIKE A MELOD)

  draw q50;
  draw q51;
  %draw q52;

  p203 := get_point 2 q52;

  p204 := mediate(p103, p203);

  p205 := p103 shifted (-offset, -offset);
  p206 := p105 shifted (offset, -offset);
  p207 := p203 shifted (offset, offset);
  p208 := p106 shifted (-offset, offset);
  p209 := mediate(p205, p206);

  q150 := p205 -- p206 -- p207 -- p208 -- cycle;
  draw q150 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{103}$", p103);
    dotlabel.bot("$p_{105}$", p105);
    dotlabel.top("$p_{106}$", p106);
    dotlabel.bot("$p_{107}$", p107);
    dotlabel.top("$p_{203}$", p203);
    dotlabel.bot("$p_{204}$", p204);
    dotlabel.bot("$p_{205}$", p205);
    dotlabel.bot("$p_{206}$", p206);
    dotlabel.top("$p_{207}$", p207);
    dotlabel.top("$p_{208}$", p208);
    dotlabel.bot("$p_{209}$", p209);
    label.top("$q_{150}$", mediate(p208, p207)) with_color red;
  else:
    draw q150;
  fi;

  % shift current_picture (12, 8);
  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

  v14 := current_picture;
  clear current_picture;

%% ** (2)

  p413 := p105 shifted (7.5, 0);

  shift Q38 by (p413 - p408);
  shift Q39 by (p413 - p408);
  shift Q40 by (p413 - p408);

  draw Q40 with_color gray;
  fill Q38 with_color gray;
  unfill Q39;
  draw Q38;
  draw Q39;

  p414 := get_point 1 Q40;
  p415 := get_point 2 Q40;
  p416 := get_point 3 Q40;
  p417 := mediate(p413, p415);

  p418 := p413 shifted (-offset, -offset);
  p419 := p414 shifted (offset, -offset);
  p420 := p415 shifted (offset, offset);
  p421 := p416 shifted (-offset, offset);

  Q41 := p418 -- p419 -- p420 -- p421 -- cycle;

  draw Q41 with_color red;

  if do_labels:
    dotlabel.bot("$p_{413}$", p413);
    dotlabel.bot("$p_{414}$", p414);
    dotlabel.top("$p_{415}$", p415);
    dotlabel.top("$p_{416}$", p416);
    dotlabel.bot("$p_{417}$", p417);
    dotlabel.bot("$p_{418}$", p418);
    dotlabel.bot("$p_{419}$", p419);
    dotlabel.top("$p_{420}$", p420);
    dotlabel.top("$p_{421}$", p421);
    label.rt("$Q_{38}$", mediate(p414, p415));
    label("$Q_{39}$", p417 shifted (1, 1.25));
    label.lft("$Q_{40}$", mediate(p413, p416)) with_color dark_gray;
    label.top("$Q_{41}$", mediate(p420, p421)) with_color red;
  fi;

  v15 := current_picture;
  clear current_picture;
  
%% ** (2) Put second figure with larger surrounding boxes together.
  
%% **** (4) L (A PRETTY GIRL)
  
  temp_picture := v9;
  shift temp_picture (-14, 14);
  current_picture += temp_picture;

%% **** (4) S (A PRETTY GIRL IS)
  
  temp_picture := v10;
  shift temp_picture (-8, 14);
  current_picture += temp_picture;

%% **** (4) K (A PRETTY GIRL IS LIK)
  
  temp_picture := v11;
  shift temp_picture (-2, 14);
  current_picture += temp_picture;

%% **** (4) M (A PRETTY GIRL IS LIKE A M)

  temp_picture := v12;
  shift temp_picture (3, 14);
  current_picture += temp_picture;

%% **** (4) O (A PRETTY GIRL IS LIKE A MELO)
  
  temp_picture := v13;
  shift temp_picture (-52, -2);
  current_picture += temp_picture;

%% **** (4) D (A PRETTY GIRL IS LIKE A MELOD)
  
  temp_picture := v14;
  shift temp_picture (12, 8);
  current_picture += temp_picture;

%% **** (4) @ (At sign)
  
  temp_picture := v15;
  shift temp_picture (12, 8);
  current_picture += temp_picture;

  
%% ** (3) End of second figure with larger surrounding boxes.
 
endfig with_projection parallel_x_y;
fig_ctr += 1;  

%% ** (2)

%% * (1) Third figure with larger surrounding boxes.

beginfig(fig_ctr);
%% ** (2)
  
  draw frame with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

%% ** (2) U (LAU)
  

  draw Q1 with_color gray;
  draw Q0;
  
  for i = 1 upto 3:
    p[223+i] := get_point (i) Q1;
  endfor;
  
  if do_labels:
    dotlabel.bot("$p_{222}$", p222);
    dotlabel.bot("$p_{224}$", p224);
    dotlabel.top("$p_{225}$", p225);
    dotlabel.top("$p_{226}$", p226);
  fi;

  p227 := p222 shifted (-offset, -offset);
  p228 := p224 shifted (offset, -offset);
  p229 := p225 shifted (offset, offset);
  p230 := p226 shifted (-offset, offset);

  p231 := mediate(p227, p229);
  p232 := mediate(p227, p228);

  if do_labels:
    dotlabel.bot("$p_{227}$", p227);
    dotlabel.bot("$p_{228}$", p228);
    dotlabel.top("$p_{229}$", p229);
    dotlabel.top("$p_{230}$", p230);
    dotlabel.bot("$p_{231}$", p231);
    dotlabel.bot("$p_{232}$", p232);
  fi;

  Q100 := p227 -- p228 -- p229 -- p230 -- cycle;

  draw Q100 with_color red;

  if do_labels:
    label.top("$Q_{100}$", mediate(p229, p230)) with_color red;
    label.rt("$Q_{0}$", mediate(get_point 6 Q0, get_point 8 Q0)) shifted (4mm, 0);
    label.top("$Q_{1}$", mediate(get_point 2 Q1, get_point 3 Q1)) shifted (0, 1mm);
  fi;

  shift current_picture (2, -1);
  v21 := current_picture;
  clear current_picture;


%% ** (2) N (LAUREN)

  draw Q3 with_color gray;
  draw Q2;


  for i = 1 upto 3:
    p[234+i] := get_point (i) Q3;
  endfor;

  p238 := mediate(p233, p235);
  p239 := mediate(p233, p236);

  p240 := p233 shifted (-offset, -offset);
  p241 := p235 shifted (offset, -offset);
  p242 := p236 shifted (offset, offset);
  p243 := p237 shifted (-offset, offset);
  p244 := mediate(p240, p241);

  Q102 := p240 -- p241 -- p242 -- p243 -- cycle;

  draw Q102 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{233}$", p233);
    dotlabel.bot("$p_{235}$", p235);
    dotlabel.top("$p_{236}$", p236);
    dotlabel.top("$p_{237}$", p237);
    dotlabel.bot("$p_{238}$", p238);
    dotlabel.bot("$p_{239}$", p239);
    label.top("$Q_{3}$", mediate(p237, p236));
    label.rt("$Q_{2}$", mediate(get_point 7 Q2, get_point 9 Q2, .25));
    dotlabel.bot("$p_{240}$", p240);
    dotlabel.bot("$p_{241}$", p241);
    dotlabel.top("$p_{242}$", p242);
    dotlabel.top("$p_{243}$", p243);
    dotlabel.bot("$p_{244}$", p244);
    label.top("$Q_{102}$", mediate(p242, p243)) with_color red;
  fi;

  shift current_picture by (p228 - p240);
  shift current_picture (5, -1);

  v22 := current_picture;
  clear current_picture;

  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

%% ** (2) C (LAURENC)

  draw Q5 with_color gray;
  draw Q4;

  p248 := get_point 1 Q5;
  p249 := get_point 2 Q5;
  p250 := mediate(p245, p249);

  p251 := p245 shifted (-offset, -offset);
  p252 := p248 shifted (offset, -offset);
  p253 := p249 shifted (offset, offset);
  p254 := p247 shifted (-offset, offset);
  p255 := mediate(p245, p248);
  p256 := mediate(p251, p252);

  Q104 := p251 -- p252 -- p253 -- p254 -- cycle;

  draw Q104 with_color red;


  
  if do_labels:
    dotlabel.bot("$p_{245}$", p245);
    dotlabel.top("$p_{247}$", p247);
    dotlabel.bot("$p_{248}$", p248);
    dotlabel.top("$p_{249}$", p249);
    drawdot p250 with_pen dot_pen;
    label.bot("$p_{250}$", p250) shifted (0, -2mm);
    dotlabel.bot("$p_{251}$", p251);
    dotlabel.bot("$p_{252}$", p252);
    dotlabel.top("$p_{253}$", p253);
    dotlabel.top("$p_{254}$", p254);
    dotlabel.bot("$p_{255}$", p255);
    dotlabel.bot("$p_{256}$", p256);
    label.bot("$Q_{4}$", get_point 4 Q4) shifted (0, -2mm);
    label.top("$Q_{104}$", mediate(p254, p253)) with_color red;
    label.lft("$Q_{5}$", mediate(get_point 1 Q5, get_point 2 Q5)) shifted (-1mm, 0);
  fi;

  shift current_picture by (p241 - p251);
  shift current_picture (9, -1);

  v23 := current_picture;
  clear current_picture;

  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

%% ** (2) F (LAURENCE F)

  draw Q7 with_color gray;
  draw Q6;

  p263 := p257 shifted (-offset, -offset);
  p264 := p258 shifted (offset, -offset);
  p265 := p259 shifted (offset, offset);
  p266 := p260 shifted (-offset, offset);
  p267 := mediate(p263, p264);

  Q106 := p263 -- p264 -- p265 -- p266 -- cycle;

  draw Q106 with_color red;
  
  
  if do_labels:
    dotlabel.bot("$p_{257}$", p257);
    dotlabel.bot("$p_{258}$", p258);
    dotlabel.top("$p_{259}$", p259);
    dotlabel.top("$p_{260}$", p260);
    dotlabel.lft("$p_{261}$", p261);
    dotlabel.bot("$p_{262}$", p262);
    dotlabel.bot("$p_{263}$", p263);
    dotlabel.bot("$p_{264}$", p264);
    dotlabel.top("$p_{265}$", p265);
    dotlabel.top("$p_{266}$", p266);
    dotlabel.bot("$p_{267}$", p267);
    
    label.rt("$Q_{6}$", mediate(get_point 4 Q6, get_point 5 Q6));
    label.lft("$Q_{7}$", mediate(get_point 1 Q7, get_point 2 Q7, .25)) shifted (-1mm, 0);
    label.top("$Q_{106}$", mediate(p265, p266)) with_color red;

  fi;

  shift current_picture (14, -1);
  v24 := current_picture;
  clear current_picture;

  % output current_picture with_projection parallel_x_y;
  % clear current_picture;

%% ** (2) B

  draw Q11 with_color gray;
  draw Q8;
  draw Q9;
  draw Q10;

  p275 := p268 shifted (-offset, -offset);
  p276 := p270 shifted (offset, -offset);
  p277 := p271 shifted (offset, offset);
  p278 := p272 shifted (-offset, offset);
  p279 := mediate(p275, p276);

  Q108 := p275 -- p276 -- p277 -- p278 -- cycle;

  draw Q108 with_color red;

  if do_labels:
    dotlabel.bot("$p_{268}$", p268);
    dotlabel.bot("$p_{270}$", p270);
    dotlabel.top("$p_{271}$", p271);
    dotlabel.top("$p_{272}$", p272);
    dotlabel.bot("$p_{275}$", p275);
    dotlabel.bot("$p_{276}$", p276);
    dotlabel.top("$p_{277}$", p277);
    dotlabel.top("$p_{278}$", p278);
    dotlabel.bot("$p_{279}$", p279);
    drawdot p273 with_pen dot_pen;
    label.bot("$p_{273}$", p273 shifted (-2mm, -5mm));
    dotlabel.bot("$p_{274}$", p274);
    label.top("$Q_{108}$", mediate(p278, p277)) with_color red;
    label.lft("$Q_{8}$", mediate(p268, p272));
    label.urt("$Q_{9}$", get_point 0 Q9);
    label.urt("$Q_{10}$", get_point 0 Q10);
  fi;


  shift current_picture (-34, -16);
  v25 := current_picture;
  clear current_picture;
  


%% ** (2) H

  draw Q13 with_color gray;
  draw Q12;

  p287 := p280 shifted (-offset, -offset);
  p288 := p282 shifted (offset, -offset);
  p289 := p283 shifted (offset, offset);
  p290 := p284 shifted (-offset, offset);
  p291 := mediate(p287, p288);

  Q112 := p287 -- p288 -- p289 -- p290 -- cycle;
  draw Q112 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{280}$", p280);
    dotlabel.bot("$p_{282}$", p282);
    dotlabel.top("$p_{283}$", p283);
    dotlabel.top("$p_{284}$", p284);
    drawdot p285 with_pen dot_pen;
    label.bot("$p_{285}$", p285 shifted (0, -6mm));
    dotlabel.bot("$p_{286}$", p286);
    label.rt("$Q_{12}$", mediate(get_point 13 Q12, get_point 14 Q12, .25));
    label.bot("$Q_{13}$", mediate(p284, p283));
    dotlabel.bot("$p_{287}$", p287);
    dotlabel.bot("$p_{288}$", p288);
    dotlabel.top("$p_{289}$", p289);
    dotlabel.top("$p_{290}$", p290);
    dotlabel.bot("$p_{291}$", p291);
    label.top("$Q_{112}$", mediate(p290, p289)) with_color red;
  fi;

  if false: % true:
    n := (size Q12) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      dotlabel.bot(s, get_point (i) Q12) with_color red;
    endfor;
  fi;

  
  
  shift current_picture (16, -4);
  v26 := current_picture;
  clear current_picture;  

%% ** (2) J

  draw Q15 with_color gray;
  draw Q14;

  p299 := p292 shifted (-offset, -offset);
  p300 := p294 shifted (offset, -offset);
  p301 := p295 shifted (offset, offset);
  p302 := p296 shifted (-offset, offset);
  p303 := mediate(p299, p300);

  Q114 := p299 -- p300 -- p301 -- p302 -- cycle;

  draw Q114 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{292}$", p292);
    dotlabel.bot("$p_{294}$", p294);
    dotlabel.top("$p_{295}$", p295);
    dotlabel.top("$p_{296}$", p296);
    dotlabel.bot("$p_{297}$", p297);
    dotlabel.bot("$p_{298}$", p298);
    dotlabel.bot("$p_{299}$", p299);
    dotlabel.bot("$p_{300}$", p300);
    dotlabel.top("$p_{301}$", p301);
    dotlabel.top("$p_{302}$", p302);
    dotlabel.bot("$p_{303}$", p303);
    label.bot("$Q_{15}$", mediate(p296, p295, .25));
    label("$Q_{14}$", p297 shifted (-5mm, 10mm));
    label.top("$Q_{114}$", mediate(p301, p302)) with_color red;
  fi;

  shift current_picture (20, -4);
  v27 := current_picture;
  clear current_picture;  

%% ** (2) Q

  draw p312 -- p313 with_color pink_rgb;
  draw Q18 with_color gray;

  draw Q16;
  draw Q17;

  p314 := p306 shifted (-offset, -offset);
  p315 := p307 shifted (offset, -offset);
  p316 := p308 shifted (offset, offset);
  p317 := p309 shifted (-offset, offset);
  p318 := mediate(p314, p315);

  Q116 := p314 -- p315 -- p316 -- p317 -- cycle;

  draw Q116 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{306}$", p306);
    dotlabel.bot("$p_{307}$", p307);
    dotlabel.top("$p_{308}$", p308);
    dotlabel.top("$p_{309}$", p309);
    dotlabel.top("$p_{310}$", p310);
    dotlabel.bot("$p_{311}$", p311);
    dotlabel.ulft("$p_{312}$", p312);
    dotlabel.urt("$p_{313}$", p313);
    dotlabel.bot("$p_{314}$", p314);
    dotlabel.bot("$p_{315}$", p315);
    dotlabel.top("$p_{316}$", p316);
    dotlabel.top("$p_{317}$", p317);
    dotlabel.bot("$p_{318}$", p318);
    label("$Q_{16}$", p310 shifted (0, 3.3cm));
    label.bot("$Q_{17}$", p310 shifted (0, -2mm));
    label.urt("$Q_{18}$", p306);
    label.top("$Q_{116}$", mediate(p316, p317)) with_color red;
  fi;

  shift current_picture (22, -4);
  v28 := current_picture;
  clear current_picture;

%% ** (2) Composite pictures.

  current_picture += v21; % U
  current_picture += v22; % N
  current_picture += v23; % C
  current_picture += v24; % F
  current_picture += v25; % B
  current_picture += v26; % H
  current_picture += v27; % J
  current_picture += v28; % Q 

  
endfig with_projection parallel_x_y;
fig_ctr += 1;

%% * (1) Fourth figure with larger surrounding boxes.

%message "fig_ctr: " & decimal fig_ctr;

beginfig(fig_ctr);

  draw frame with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;
  
%% ** (2) V 

  draw Q20 with_color gray;
  draw Q19;

  p326 := p319 shifted (-offset, -offset);
  p327 := p321 shifted (offset, -offset);
  p328 := p322 shifted (offset, offset);
  p329 := p323 shifted (-offset, offset);
  p330 := mediate(p326, p327);

  Q119 := p326 -- p327 -- p328 -- p329 -- cycle;

  draw Q119 with_color red;

  if do_labels:
    dotlabel.bot("$p_{319}$", p319);
    dotlabel.bot("$p_{321}$", p321);
    dotlabel.top("$p_{322}$", p322);
    dotlabel.top("$p_{323}$", p323);
    dotlabel.top("$p_{324}$", p324);
    dotlabel.bot("$p_{325}$", p325);
    dotlabel.bot("$p_{326}$", p326);
    dotlabel.bot("$p_{327}$", p327);
    dotlabel.top("$p_{328}$", p328);
    dotlabel.top("$p_{329}$", p329);
    dotlabel.bot("$p_{330}$", p330);
    label.top("$Q_{119}$", mediate(p328, p329)) with_color red;
    label("$Q_{19}$", p324 shifted (-5mm, 2cm));
    label.top("$Q_{20}$", mediate(p323, p322));
  fi;


  
  shift current_picture (-28, 10);
  v29 := current_picture;
  clear current_picture;

%% ** (2) W 

  draw Q22 with_color gray;
  draw Q21;


  p338 := p332 shifted (-offset, -offset);
  p339 := p333 shifted (offset, -offset);
  p340 := p334 shifted (offset, offset);
  p341 := p335 shifted (-offset, offset);
  p342 := mediate(p338, p339);

  Q122 := p338 -- p339 -- p340 -- p341 -- cycle;

  draw Q122 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{332}$", p332);
    dotlabel.bot("$p_{333}$", p333);
    dotlabel.top("$p_{334}$", p334);
    dotlabel.top("$p_{335}$", p335);
    dotlabel.bot("$p_{336}$", p336);
    dotlabel.bot("$p_{337}$", p337);
    dotlabel.bot("$p_{338}$", p338);
    dotlabel.bot("$p_{339}$", p339);
    dotlabel.top("$p_{340}$", p340);
    dotlabel.top("$p_{341}$", p341);
    dotlabel.bot("$p_{342}$", p342);
    label.urt("$Q_{21}$", mediate(get_point 17 Q21, get_point 18 Q21, .333));
    label.top("$Q_{22}$", mediate(p335, p334, .275));
    label.top("$Q_{122}$", mediate(p340, p341)) with_color red;
  fi;

  % message "magnitude (p340 - p341):";
  % show magnitude (p340 - p341);
  % % >> 15.3208
  % pause;

  % message "magnitude (p334 - p335):";
  % show magnitude (p334 - p335);
  % % >> 12.3208
  % pause;

  
  if false: % true:
    n := (size Q21) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      dotlabel.bot(s, get_point (i) Q21) with_color red;
    endfor;
  fi;


  shift current_picture (-28, 10);
  shift current_picture by (p321 - p332);
  shift current_picture (7, 0);
  v30 := current_picture;
  clear current_picture;


%% ** (2) X

  draw Q24 with_color gray;
  draw Q23;

  p350 := p344 shifted (-offset, -offset);
  p351 := p345 shifted (offset, -offset);
  p352 := p346 shifted (offset, offset);
  p353 := p347 shifted (-offset, offset);

  Q123 := p350 -- p351 -- p352 -- p353 -- cycle;

  draw Q123 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{344}$", p344);
    dotlabel.bot("$p_{345}$", p345);
    dotlabel.top("$p_{346}$", p346);
    dotlabel.top("$p_{347}$", p347);
    dotlabel.bot("$p_{348}$", p348);
    dotlabel.bot("$p_{349}$", p349);
    dotlabel.bot("$p_{350}$", p350);
    dotlabel.bot("$p_{351}$", p351);
    dotlabel.top("$p_{352}$", p352);
    dotlabel.top("$p_{353}$", p353);
    label.lft("$Q_{23}$", p348 shifted (-1.5, 0));
    label.lft("$Q_{24}$", mediate(p345, p346)) with_color dark_gray;
    label.top("$Q_{123}$", mediate(p353, p352)) with_color red;
    dotlabel.bot("$p_{354}$", mediate(p350, p351));
  fi;

  shift current_picture (-5, 10);
  v31 := current_picture;
  clear current_picture;
  
%% ** (2) Z

  
  draw Q26 with_color gray;
  draw Q25;

  p361 := p355 shifted (-offset, -offset);
  p362 := p356 shifted (offset, -offset);
  p363 := p357 shifted (offset, offset);
  p364 := p358 shifted (-offset, offset);
  p365 := mediate(p361, p362);

  Q125 := p361 -- p362 -- p363 -- p364 -- cycle;

  draw Q125 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{355}$", p355);
    dotlabel.bot("$p_{356}$", p356);
    dotlabel.top("$p_{357}$", p357);
    dotlabel.top("$p_{358}$", p358);
    dotlabel.bot("$p_{359}$", p359);
    dotlabel.bot("$p_{360}$", p360);
    dotlabel.bot("$p_{361}$", p361);
    dotlabel.bot("$p_{362}$", p362);
    dotlabel.top("$p_{363}$", p363);
    dotlabel.top("$p_{364}$", p364);
    dotlabel.bot("$p_{365}$", p365);
    label.rt("$Q_{25}$", p360 shifted(0, 2));
    label.rt("$Q_{26}$", mediate(p355, p358, .6667));
    label.top("$Q_{125}$", mediate(p363, p364)) with_color red;
  fi;
    
  if false: % true:
    n := (size Q25) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      dotlabel.bot(s, get_point (i) Q25) with_color red;
    endfor;
  fi;

  shift current_picture (2, 4);
  v32 := current_picture;
  clear current_picture;
  
%% ** (2) ? (Question mark) (Special characters 1)

  draw Q29 with_color gray;
  draw Q27;
  draw Q28;

  p372 := p366 shifted (-offset, -offset);
  p373 := p367 shifted (offset, -offset);
  p374 := p368 shifted (offset, offset);
  p375 := p369 shifted (-offset, offset);
  p376 := mediate(p372, p373);

  Q127 := p372 -- p373 -- p374 -- p375 -- cycle;

  draw Q127 with_color red;

  
  if do_labels:
    dotlabel.bot("$p_{366}$", p366);
    dotlabel.bot("$p_{367}$", p367);
    dotlabel.top("$p_{368}$", p368);
    dotlabel.top("$p_{369}$", p369);
    dotlabel.bot("$p_{370}$", p370);
    dotlabel.bot("$p_{371}$", p371);
    dotlabel.bot("$p_{372}$", p372);
    dotlabel.bot("$p_{373}$", p373);
    dotlabel.top("$p_{374}$", p374);
    dotlabel.top("$p_{375}$", p375);
    dotlabel.bot("$p_{376}$", p376);
    label.top("$Q_{127}$", mediate(p375, p374)) with_color red;
  fi;

  shift current_picture (3.5, 4);
  v33 := current_picture;
  clear current_picture;

  
%% ** (2) ! (Exclamation point) (Special characters 2)

  draw Q32 with_color gray;
  draw Q30;
  draw Q31;

  p383 := p379 shifted (-offset, -offset);
  p384 := p380 shifted (offset, -offset);
  p385 := p381 shifted (offset, offset);
  p386 := p382 shifted (-offset, offset);
  p387 := mediate(p383, p384);
  p388 := mediate(p383, p385);

  Q130 := p383 -- p384 -- p385 -- p386 -- cycle;

  draw Q130 with_color red;
  
  if do_labels:
    for i = 379 upto 382:
      drawdot p[i] with_pen dot_pen;
    endfor;

    label.llft("$p_{379}$", p379 shifted (4mm, -2mm));
    label.lrt("$p_{380}$",  p380 shifted (-4mm, -2mm));
    label.urt("$p_{381}$", p381 shifted (-4mm, 1mm));
    label.ulft("$p_{382}$", p382 shifted (4mm, 1mm));
    dotlabel.bot("$p_{383}$", p383);
    dotlabel.bot("$p_{384}$", p384);
    dotlabel.urt("$p_{385}$", p385);
    dotlabel.ulft("$p_{386}$", p386);
    dotlabel.bot("$p_{387}$", p387);
    dotlabel.bot("$p_{388}$", p388);
    label.top("$Q_{130}$", mediate(p386, p385)) with_color red;
    label("$Q_{30}$", p388 shifted (0, 2));
    label("$Q_{31}$", p379 shifted (.6, .7));
    label("$Q_{32}$", p380 shifted (.75, 2));
  fi;

  shift current_picture (-7, 4);
  v34 := current_picture;
  clear current_picture;


%% ** (2) ' (Apostrophe) (Special characters 3)

  draw Q34 with_color gray;
  draw Q33;

  p393 := mediate(p389, p391);

  p394 := p389 shifted (-offset, -offset);
  p395 := p390 shifted (offset, -offset);
  p396 := p391 shifted (offset, offset);
  p397 := p392 shifted (-offset, offset);

  Q133 := p394 -- p395 -- p396 -- p397 -- cycle;

  draw Q133 with_color red;
  
  if do_labels:
    dotlabel.llft("$p_{389}$", p389);
    dotlabel.lrt("$p_{390}$", p390);
    dotlabel.urt("$p_{391}$", p391);
    dotlabel.ulft("$p_{392}$", p392);
    dotlabel.bot("$p_{393}$", p393);
    dotlabel.bot("$p_{394}$", p394);
    dotlabel.bot("$p_{395}$", p395);
    dotlabel.urt("$p_{396}$", p396);
    dotlabel.ulft("$p_{397}$", p397);
    label.top("$Q_{133}$", mediate(p397, p396));
    label("$Q_{33}$", mediate(p392, p391) shifted (0, -.6));
    label.rt("$Q_{34}$", mediate(p390, p391, 2/3)) with_color dark_gray;
  fi;

  shift current_picture (-5, 4);
  
  v35 := current_picture;
  clear current_picture;


%% ** (2) # (Hash mark (number sign))

  draw Q37 with_color gray;
  draw Q35;
  draw Q36;

  p404 := p399 shifted (-offset, -offset);
  p405 := p400 shifted (offset, -offset);
  p406 := p401 shifted (offset, offset);
  p407 := p402 shifted (-offset, offset);

  Q135 := p404 -- p405 -- p406 -- p407 -- cycle;

  draw Q135 with_color red;
  

  if do_labels:
    dotlabel.bot("$p_{399}$", p399);
    dotlabel.bot("$p_{400}$", p400);
    dotlabel.top("$p_{401}$", p401);
    dotlabel.top("$p_{402}$", p402);
    dotlabel.bot("$p_{403}$", p403);
    dotlabel.bot("$p_{404}$", p404);
    dotlabel.bot("$p_{405}$", p405);
    dotlabel.top("$p_{406}$", p406);
    dotlabel.top("$p_{407}$", p407);
    label.ulft("$Q_{35}$", get_point 13 Q35);
    label.top("$Q_{36}$", p403);
    label.lrt("$Q_{37}$", p402) with_color dark_gray;
    label.top("$Q_{135}$", mediate(p406, p407)) with_color red;
  fi;

  
  if false: % true:
    n := (size Q35) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q35 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q35) with_color red;
    endfor;
  fi;

  shift current_picture (45, -18);
  
  v36 := current_picture;
  clear current_picture;
  
  
%% ** (2) Composite pictures.

  current_picture += v29; % V
  current_picture += v30; % W
  current_picture += v31; % X
  current_picture += v32; % Z
  current_picture += v33; % ? (Question mark)
  current_picture += v34; % ! (Exclamation point)
  current_picture += v35; % ' (Apostrophe)
  current_picture += v36; % # (Hash mark (number sign))
  
  
endfig with_projection parallel_x_y;
fig_ctr += 1;

%% * (1)

beginfig(fig_ctr);

%% ** (2)
  
  draw frame with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

%% ** (2) Omega (Greek letter Omega, uppercase)

  glyph Omega;
  Omega := get_glyph 10 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Omega;

  scale qv (10, 10);
  
  Q42 := qv0;
  Q43 := qv1;

  p422 := p409 shifted (8, 0);

  p423 := (xpart get_point 17 Q42, ypart get_point 18 Q42);

  shift Q42 by (p422 - p423);
  shift Q43 by (p422 - p423);

  p423 := get_point 1 Q43;
  p424 := get_point 2 Q43;
  p425 := (xpart p422, ypart p424);
  p426 := mediate(p422, p424);

  
  draw Q43 with_color gray;
  draw Q42;

  p427 := p422 shifted (-offset, -offset);
  p428 := p423 shifted (offset, -offset);
  p429 := p424 shifted (offset, offset);
  p430 := p425 shifted (-offset, offset);

  Q44 := p427 -- p428 -- p429 -- p430 -- cycle;

  draw Q44 with_color red;

  
  if do_labels:
    dotlabel.bot("$p_{422}$", p422);
    dotlabel.bot("$p_{423}$", p423);
    dotlabel.top("$p_{424}$", p424);
    dotlabel.top("$p_{425}$", p425);
    dotlabel.bot("$p_{426}$", p426);
    dotlabel.bot("$p_{427}$", p427);
    dotlabel.bot("$p_{428}$", p428);
    dotlabel.top("$p_{429}$", p429);
    dotlabel.top("$p_{430}$", p430);
    label.bot("$Q_{42}$", get_point 23 Q42);
    label.lrt("$Q_{43}$", get_point 5 Q42) shifted (0, 0) with_color dark_gray;
    label.top("$Q_{44}$", mediate(p429, p430)) with_color red;
    label.bot("$c_{har10~(Omega)}$", mediate(p427, p428) shifted (0, -.5));
  fi;

  shift current_picture (-28, 1);
  
  v37 := current_picture;
  clear current_picture;

%% ** (2) Psi (Greek letter Psi, uppercase)

  % Skipping the separate picture without the red enclosing box,
  % as done for the other glyphs.  These pictures aren't needed for
  % anything.  LDF 2024.02.25.
    
  glyph Psi;
  Psi := get_glyph 9 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Psi;

  scale qv (10, 10);
  
  Q45 := qv0;
  Q46 := qv1;


  p431 := (p423 shifted (-28, 1)) shifted (8, 0);

  p432 := (xpart get_point 8 Q45, ypart get_point 13 Q45);

  shift Q45 by (p431 - p432);
  shift Q46 by (p431 - p432);

  p432 := get_point 1 Q46;
  p433 := get_point 2 Q46;
  p434 := (xpart p431, ypart p433);
  p435 := mediate(p431, p433);

  Q46 := p431 -- p432 -- p433 -- p434 -- cycle;
  
  draw Q46 with_color gray;
  draw Q45;

  p436 := p431 shifted by (-offset, -offset);
  p437 := p432 shifted by (offset, -offset);
  p438 := p433 shifted by (offset, offset);
  p439 := p434 shifted by (-offset, offset);

  Q47 := p436 -- p437 -- p438 -- p439 -- cycle;

  draw Q47 with_color red;
  
  
  if do_labels:
    dotlabel.bot("$p_{431}$", p431);
    dotlabel.bot("$p_{432}$", p432);
    dotlabel.top("$p_{433}$", p433);
    dotlabel.top("$p_{434}$", p434);

    dotlabel.bot("$p_{436}$", p436);
    dotlabel.bot("$p_{437}$", p437);
    dotlabel.top("$p_{438}$", p438);
    dotlabel.top("$p_{439}$", p439);
    
    
    dotlabel.bot("$p_{435}$", p435);
    label.urt("$Q_{45}$", p431 shifted (.333, .5));
    label.lft("$Q_{46}$", p432 shifted (-.25, .5)) with_color dark_gray;
    label.top("$Q_{47}$", mediate(p438, p439)) with_color red;
    label.bot("$c_{har9~(Psi)}$", mediate(p436, p437) shifted (0, -.5));
  fi;

  if false: % true:
    n := (size Q45) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q45 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q45) with_color red;
    endfor;
  fi;

  v38 := current_picture;
  clear current_picture;
  
%% ** (2) Delta (Greek letter Delta, uppercase)

  glyph Delta;
  Delta := get_glyph 1 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Delta;

  scale qv (10, 10);
  
  Q48:= qv0;
  Q49:= qv1;
  Q50:= qv2;

  p440 := p432 shifted (8, 0);
  p441 := (xpart get_point 5 Q48, ypart get_point 6 Q48);

  shift Q48 by (p440 - p441);
  shift Q49 by (p440 - p441);
  shift Q50 by (p440 - p441);


  p441 := get_point 1 Q50;
  p442 := get_point 2 Q50;
  p443 := (xpart p440, ypart p442);
  p444 := mediate(p440, p442);
  
  Q50 := p440 -- p441 -- p442 -- p443 -- cycle;  

  draw Q50 with_color gray;
  draw Q48;
  draw Q49;

  p445 := p440 shifted (-offset, -offset);
  p446 := p441 shifted (offset, -offset);
  p447 := p442 shifted (offset, offset);
  p448 := p443 shifted (-offset, offset);

  Q51 := p445 -- p446 -- p447 -- p448 -- cycle;

  draw Q51 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{440}$", p440);
    dotlabel.bot("$p_{441}$", p441);
    dotlabel.top("$p_{442}$", p442);
    dotlabel.top("$p_{443}$", p443);
    dotlabel.bot("$p_{444}$", p444);
    dotlabel.bot("$p_{445}$", p445);
    dotlabel.bot("$p_{446}$", p446);
    dotlabel.top("$p_{447}$", p447);
    dotlabel.top("$p_{448}$", p448);
    label.ulft("$Q_{48}$", mediate(get_point 3 Q48, get_point 4 Q48));
    label.urt("$Q_{49}$", get_point 2 Q49) shifted (5mm, 0);
    label.lft("$Q_{50}$", mediate(get_point 1 Q50, get_point 2 Q50, .6667))
      with_color dark_gray;
    label.top("$Q_{51}$", mediate(p447, p448)) with_color red;
    label.bot("$c_{har1~(Delta)}$", mediate(p445, p446) shifted (0, -.5));
  fi;
  

  if false: % true:
    n := (size Q48) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q48 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q48) with_color red;
    endfor;
  fi;

  v39 := current_picture;
  clear current_picture;

%% ** (2) Two ('2') (Numeral 2)

  glyph Two;
  Two := get_glyph 50 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Two;

  scale qv (10, 10);
  
  Q52:= qv0;
  Q53:= qv1;


  p449 := p0 shifted (4, 4);

  p450 := (xpart get_point 16 Q52, ypart get_point 3 Q52);


  shift Q52 by (p449 - p450);
  shift Q53 by (p449 - p450);

  p450 := get_point 1 Q53;
  p451 := get_point 2 Q53;
  p452 := (xpart p449, ypart p451);
  p453 := mediate(p449, p451);

  Q53 := p449 -- p450 -- p451 -- p452 -- cycle;
  
  draw Q53 with_color gray;
  draw Q52;

  p454 := p449 shifted (-offset, -offset);
  p455 := p450 shifted (offset, -offset);
  p456 := p451 shifted (offset, offset);
  p457 := p452 shifted (-offset, offset);

  Q54 := p454 -- p455 -- p456 -- p457 -- cycle;

  draw Q54 with_color red;

  if do_labels:
    dotlabel.bot("$p_{449}$", p449);
    dotlabel.bot("$p_{450}$", p450);
    dotlabel.top("$p_{451}$", p451);
    dotlabel.top("$p_{452}$", p452);
    dotlabel.lft("$p_{453}$", p453);
    dotlabel.bot("$p_{454}$", p454);
    dotlabel.bot("$p_{455}$", p455);
    dotlabel.top("$p_{456}$", p456);
    dotlabel.top("$p_{457}$", p457);
    label.bot("$Q_{52}$", get_point 20 Q52) shifted (0, -.25);
    label.urt("$Q_{53}$", get_point 9 Q52) shifted (-.3, .5) with_color dark_gray;
    label.top("$Q_{54}$", mediate(p456, p457)) with_color red;
  fi;
  

  if false: % true:
    n := (size Q52) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q52 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q52) with_color red;
    endfor;
  fi;

  v40 := current_picture;
  clear current_picture;


  
%% ** (2) Asterisk

  glyph Asterisk;
  Asterisk := get_glyph 42 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Asterisk;

  scale qv (10, 10);
  
  Q55:= qv0;
  Q56:= qv1;
  
  p458 := p450 shifted (8, 0);
  p459 := (xpart get_point 19 Q55, ypart get_point 0 Q56);

  shift Q55 by (p458 - p459);
  shift Q56 by (p458 - p459);

  p459 := get_point 1 Q56;

  p460 := get_point 2 Q56;
  p461 := (xpart p458, ypart p460);
  p462 := mediate(p458, p460);
  
  Q56 := p458 -- p459 -- p460 -- p461 -- cycle;

  draw Q56 with_color gray;
  fill Q55 with_color gray;
  draw Q55;

  p463 :=   p458 shifted (-offset, -offset);
  p464 :=   p459 shifted (offset, -offset);
  p465 :=   p460 shifted (offset, offset);
  p466 :=   p461 shifted (-offset, offset);

  Q57 := p463 -- p464 -- p465 -- p466 -- cycle;

  draw Q57 with_color red;
    

  if do_labels:
    dotlabel.bot("$p_{458}$", p458);
    dotlabel.bot("$p_{459}$", p459);
    dotlabel.top("$p_{460}$", p460);
    dotlabel.top("$p_{461}$", p461);
    dotlabel.bot("$p_{462}$", p462);
    dotlabel.bot("$p_{463}$", p463);
    dotlabel.bot("$p_{464}$", p464);
    dotlabel.top("$p_{465}$", p465);
    dotlabel.top("$p_{466}$", p466);
    label("$Q_{55}$", p462 shifted (0, -1.75));
    label.ulft("$Q_{56}$", p459) with_color dark_gray;
    label.top("$Q_{57}$", mediate(p465, p466)) with_color red;
  fi;

  if false: % true:
    n := (size Q55) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q55 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q55) with_color red;
    endfor;
  fi;

  v42 := current_picture;
  clear current_picture;

%% ** (2) O_slash 

  glyph O_slash;
  O_slash := get_glyph 31 from "cmssbx10";

  clear qv;
  
  qv := get_paths from O_slash;

  scale qv (10, 10);
  
  Q61 := qv0;
  Q62 := qv1;
  Q63 := qv2;
  Q64 := qv3;

  p476 := p468 shifted (8, 0);
  p477 := (xpart get_point 10 Q61, ypart get_point 14 Q61);

  for i = 61 upto 64:
    shift Q[i] by (p476 - p477);
  endfor;

  p477 := get_point 1 Q64;
  p478 := get_point 2 Q64;
  p479 := (xpart p476, ypart p478);
  p480 := mediate(p476, p478);

  Q64 := p476 -- p477 -- p478 -- p479 -- cycle;
  
  draw Q64 with_color gray;
  draw Q61;
  draw Q62;
  draw Q63;

  p481 := p476 shifted (-offset, -offset);
  p482 := p477 shifted (offset, -offset);
  p483 := p478 shifted (offset, offset);
  p484 := p479 shifted (-offset, offset);

  Q65 := p481 -- p482 -- p483 -- p484 -- cycle;

  draw Q65 with_color red;

  if do_labels:
    dotlabel.bot("$p_{476}$", p476);
    dotlabel.bot("$p_{477}$", p477);
    dotlabel.top("$p_{478}$", p478);
    dotlabel.top("$p_{479}$", p479);
    dotlabel.bot("$p_{480}$", p480);
    dotlabel.bot("$p_{481}$", p481);
    dotlabel.bot("$p_{482}$", p482);
    dotlabel.top("$p_{483}$", p483);
    dotlabel.top("$p_{484}$", p484);
    label.lrt("$Q_{61}$", p479 shifted (.2, -.3));
    label.ulft("$Q_{62}$", p480 shifted (-.4, .2));
    label.lrt("$Q_{63}$", p480 shifted (.4, -1));
    label.top("$Q_{64}$", mediate(p479, p478)) with_color dark_gray;
    label.top("$Q_{65}$", mediate(p483, p484)) with_color red;
  fi;
    
  if false: % true:
    n := (size Q61) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q61 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q61) with_color red;
    endfor;
  fi;

  v43 := current_picture;
  clear current_picture;

  
  
%% ** (2)

  current_picture += v37;
  current_picture += v38;
  current_picture += v39;
  current_picture += v40;
  current_picture += v41;
  current_picture += v42;
  current_picture += v43; % O-slash
 
  
endfig with_projection parallel_x_y;
fig_ctr += 1;


%% * (1)

beginfig(fig_ctr);

%% ** (2)
  
  draw frame with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

%% ** (2) Upsilon 

  glyph Upsilon;
  Upsilon := get_glyph 7 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Upsilon;

  scale qv (10, 10);
  
  Q66 := qv0;
  Q67 := qv1;

  p485 := p3 shifted (4, -12);

  p486 := (xpart get_point 10 Q66, ypart get_point 17 Q66);

  
  shift Q66 by (p485 - p486);
  shift Q67 by (p485 - p486);

  p486 := get_point 1 Q67;
  p487 := get_point 2 Q67;
  p488 := (xpart p485, ypart p487);
  p489 := mediate(p485, p487);

  Q67 := p485 -- p486 -- p487 -- p488  -- cycle;
  
  draw Q67 with_color gray;
  draw Q66;

  p490 := p485 shifted (-offset, -offset);
  p491 := p486 shifted (offset, -offset);
  p492 := p487 shifted (offset, offset);
  p493 := p488 shifted (-offset, offset);

  Q68 := p490 -- p491 -- p492 -- p493 -- cycle;

  draw Q68 with_color red;

  if do_labels:
    dotlabel.bot("$p_{485}$", p485);
    dotlabel.bot("$p_{486}$", p486);
    dotlabel.top("$p_{487}$", p487);
    dotlabel.top("$p_{488}$", p488);
    dotlabel.bot("$p_{489}$", p489);
    dotlabel.bot("$p_{490}$", p490);
    dotlabel.bot("$p_{491}$", p491);
    dotlabel.top("$p_{492}$", p492);
    dotlabel.top("$p_{493}$", p493);
    label("$Q_{66}$", p489 shifted (-2.25, 1));
    label.urt("$Q_{67}$", p485 shifted (0, 2)) with_color dark_gray;
    label.top("$Q_{68}$", mediate(p492, p493)) with_color red;
    label.bot("$c_{har7~(Upsilon)}$", mediate(p490, p491) shifted (0, -.5));
  fi;
    
  if false: % true:
    n := (size Q66) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q66 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q66) with_color red;
    endfor;
  fi;

  v44 := current_picture;
  clear current_picture;

%% ** (2) Sigma, Greek letter, uppercase

  glyph Sigma;
  Sigma := get_glyph 6 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Sigma;

  scale qv (10, 10);
  
  Q76 := qv0;
  Q77 := qv1;

  p508 := p500 shifted (8, 0);
  p509 := (xpart get_point 14 Q76, ypart get_point 15 Q76);

  shift Q76 by (p508 - p509);
  shift Q77 by (p508 - p509);

  p509 := get_point 1 Q77;
  p510 := get_point 2 Q77;
  p511 := (xpart p508, ypart p510);

  Q77 := p508 -- p509 -- p510 -- p511 -- cycle;

  p512 := p508 shifted (-offset, -offset);
  p513 := p509 shifted (offset, -offset);
  p514 := p510 shifted (offset, offset);
  p515 := p511 shifted (-offset, offset);
  p516 := mediate(p508, p510);

  Q78 := p512 -- p513 -- p514 -- p515 -- cycle;

  draw Q77 with_color gray;
  draw Q76;

  draw Q78 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{508}$", p508);
    dotlabel.bot("$p_{509}$", p509);
    dotlabel.top("$p_{510}$", p510);
    dotlabel.top("$p_{511}$", p511);
    dotlabel.bot("$p_{512}$", p512);
    dotlabel.bot("$p_{513}$", p513);
    dotlabel.top("$p_{514}$", p514);
    dotlabel.top("$p_{515}$", p515);
    dotlabel.bot("$p_{516}$", p516);
    label.bot("$Q_{76}$", mediate(get_point 4 Q76, get_point 5 Q76, .6667));
    label.lft("$Q_{77}$", mediate(p509, p510)) with_color dark_gray;
    label.top("$Q_{78}$", mediate(p514, p515)) with_color red;
    label.bot("$c_{har6~(Sigma)}$", mediate(p512, p513) shifted (0, -.5));
  fi;
  
  if false: % true:
    n := (size Q76) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q76 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q76) with_color red;
    endfor;
  fi;

%% ** (2) Phi, Greek letter, uppercase

  glyph Phi;
  Phi := get_glyph 8 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Phi;

  scale qv (10, 10);
  
  Q79 := qv0;
  Q80 := qv1;
  Q81 := qv2;
  Q82 := qv3;

    
  p517 := p485 shifted (0, -15);
  p518 := (xpart get_point 5 Q79, ypart get_point 8 Q79);
  
  for i = 79 upto 82:
    shift Q[i] by (p517 - p518);
  endfor;

  p518 := get_point 1 Q82;
  p519 := get_point 2 Q82;
  p520 := (xpart p517, ypart p519);
  p521 := mediate(p517, p519);

  Q82 := p517 -- p518 -- p519 -- p520 -- cycle;
  
  draw Q82 with_color gray;
  draw Q79;
  draw Q80;
  draw Q81;

  p522 := p517 shifted (-offset, -offset);
  p523 := p518 shifted (offset, -offset);
  p524 := p519 shifted (offset, offset);
  p525 := p520 shifted (-offset, offset);

  Q83 := p522 -- p523 -- p524 -- p525 --cycle;

  draw Q83 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{517}$", p517);
    dotlabel.bot("$p_{518}$", p518);
    dotlabel.top("$p_{519}$", p519);
    dotlabel.top("$p_{520}$", p520);
    dotlabel.bot("$p_{521}$", p521);
    dotlabel.bot("$p_{522}$", p522);
    dotlabel.bot("$p_{523}$", p523);
    dotlabel.top("$p_{524}$", p524);
    dotlabel.top("$p_{525}$", p525);
    label("$Q_{79}$", p520 shifted (1.25, -1));
    label("$Q_{80}$", p521 shifted (-1.5, 1));
    label("$Q_{81}$", p521 shifted (1.5, -1));
    label.top("$Q_{83}$", mediate(p524, p525)) with_color red;
    label.bot("$c_{har8~(Phi)}$", mediate(p522, p523) shifted (0, -.5));
  fi;
  
  if false: % true:
    n := (size Q79) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q79 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q79) with_color red;
    endfor;
  fi;

%% ** (2) Plus sign

  glyph Plus;
  Plus := get_glyph 43 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Plus;

  scale qv (10, 10);
  
  Q84 := qv0;
  Q85 := qv1;
  

  p526 := p518 shifted (8, 0);
  p527 := (xpart get_point 10 Q84, ypart get_point 14 Q84);

  shift Q84 by (p526 - p527);
  shift Q85 by (p526 - p527);

  p527 := get_point 1 Q85;
  p528 := get_point 2 Q85;
  p529 := (xpart p526, ypart p528);
  p530 := mediate(p526, p528);

  Q85 := p526 -- p527 -- p528 -- p529 -- cycle;

  draw Q85 with_color gray;
  draw Q84;

  p531 := p526 shifted (-offset, -offset);
  p532 := p527 shifted (offset, -offset);
  p533 := p528 shifted (offset, offset);
  p534 := p529 shifted (-offset, offset);

  Q86 := p531 -- p532 -- p533 -- p534 -- cycle;

  draw Q86 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{526}$", p526);
    dotlabel.bot("$p_{527}$", p527);
    dotlabel.top("$p_{528}$", p528);
    dotlabel.top("$p_{529}$", p529);
    dotlabel.bot("$p_{530}$", p530);
    dotlabel.bot("$p_{531}$", p531);
    dotlabel.bot("$p_{532}$", p532);
    dotlabel.top("$p_{533}$", p533);
    dotlabel.top("$p_{534}$", p534);
    label.urt("$Q_{84}$", p530 shifted (1, 1));
    label.lrt("$Q_{85}$", p529 shifted (.25, -.25)) with_color dark_gray;
    label.top("$Q_{86}$", mediate(p534, p533)) with_color red;
  fi;

  if false: % true:
    n := (size Q84) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q84 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q84) with_color red;
    endfor;
  fi;



%% ** (2)  
  
endfig with_projection parallel_x_y;
fig_ctr += 1;

%% * (1)

beginfig(fig_ctr);

%% ** (2)
  
  draw frame with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

%% ** (2) Xi (Greek letter, uppercase)

  glyph Xi;
  Xi := get_glyph 4 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Xi;

  scale qv (10, 10);
  
  Q95 := qv0;
  Q96 := qv1;
  Q97 := qv2;
  Q98 := qv3;

  p553 := (xpart get_point 8 Q96, ypart get_point 1 Q96);

  for i = 95 upto 98:
    shift Q[i] by (p485 - p553);
  endfor;

  p553 := get_point 1 Q98;
  p554 := get_point 2 Q98;
  p555 := (xpart p485, ypart p554);
  p556 := mediate(p485, p554);

  Q98 := p485 -- p553 -- p554 -- p555 -- cycle;
  
  draw Q98 with_color gray;
  draw Q95;
  draw Q96;
  draw Q97;

  p557 := p485 shifted (-offset, -offset);
  p558 := p553 shifted (offset, -offset);
  p559 := p554 shifted (offset, offset);
  p560 := p555 shifted (-offset, offset);

  Q99 := p557 -- p558 -- p559 -- p560 -- cycle;

  draw Q99 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{485}$", p485);
    dotlabel.bot("$p_{553}$", p553);
    dotlabel.top("$p_{554}$", p554);
    dotlabel.top("$p_{555}$", p555);
    drawdot p556 with_pen dot_pen;
    label.lft("$p_{556}$", p556 shifted (-2mm, 0));
    dotlabel.bot("$p_{557}$", p557);
    dotlabel.bot("$p_{558}$", p558);
    dotlabel.top("$p_{559}$", p559);
    dotlabel.top("$p_{560}$", p560);
    label.rt("$Q_{95}$", p556 shifted (4mm, 0));
    label.bot("$Q_{96}$", (get_point 6 Q96) shifted (0, -3mm));
    label.top("$Q_{97}$", (get_point 6 Q97) shifted (0, 2mm));
    label.lft("$Q_{98}$", mediate(p554, p553, .3)) with_color dark_gray;
    label.top("$Q_{99}$", mediate(p559, p560)) with_color red;
    label.bot("$c_{har4~(Xi)}$", mediate(p557, p558) shifted (0, -.5));
  fi;
  
  if false: % true:
    n := (size Q95) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q95 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q95) with_color red;
    endfor;
  fi;

  if false: % true:
    n := (size Q96) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q96 with_color dark_green with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q96) with_color dark_green;
    endfor;
  fi;

  if false: % true:
    n := (size Q97) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q97 with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q97);
    endfor;
  fi;
  
  
%% ** (2) Theta, Greek letter, uppercase

  glyph Theta;
  Theta := get_glyph 2 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Theta;

  scale qv (10, 10);
  
  Q103 := qv0;
  Q104 := qv1;
  Q105 := qv2;
  Q106 := qv3;

  p570 := p562 shifted (8, 0);
  
  p571 := (xpart get_point 2 Q104, ypart get_point 3 Q104);

  for i = 103 upto 106:
    shift Q[i] by (p570 - p571);
  endfor;

  p571 := get_point 1 Q106;
  p572 := get_point 2 Q106;
  p573 := (xpart p570, ypart p572);
  p574 := mediate(p570, p572);

  
  Q106 := p570 -- p571 -- p572 -- p573 -- cycle;

  
  draw Q103;
  draw Q104 with_color dark_green;
  draw Q105 with_color cyan;
  draw Q106 with_color magenta;

  p575 := p570 shifted (-offset, -offset);
  p576 := p571 shifted (offset, -offset);
  p577 := p572 shifted (offset, offset);
  p578 := p573 shifted (-offset, offset);

  Q107 := p575 -- p576 -- p577 -- p578 -- cycle;

  draw Q107 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{570}$", p570);
    dotlabel.bot("$p_{571}$", p571);
    dotlabel.top("$p_{572}$", p572);
    dotlabel.top("$p_{573}$", p573);
    dotlabel.bot("$p_{574}$", p574);
    dotlabel.bot("$p_{575}$", p575);
    dotlabel.bot("$p_{576}$", p576);
    dotlabel.top("$p_{577}$", p577);
    dotlabel.top("$p_{578}$", p578);
    label("$Q_{103}$", p574 shifted (0, 1.5));
    label.rt("$Q_{104}$", get_point 2 Q104) with_color dark_green;
    label("$Q_{105}$", mediate(p573, p571, .25) shifted (-2mm, 0)) with_color cyan;
    label.rt("$Q_{106}$", mediate(p570, p573, .1)) with_color magenta;
    label.top("$Q_{107}$", mediate(p577, p578)) with_color red;
    label.bot("$c_{har2~(Theta)}$", mediate(p575, p576) shifted (0, -.5));
  fi;

  if false: % true:
    n := (size Q104) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q104 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q104) with_color red;
    endfor;
  fi;
  
%% ** (2) Gamma, Greek letter, uppercase

  glyph Gamma;
  Gamma := get_glyph 0 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Gamma;

  scale qv (10, 10);
  
  Q118 := qv0;
  Q119 := qv1;

  p605 := p597 shifted (8, 0);
  p606 := (xpart get_point 8 Q118, ypart get_point 9 Q118);

  shift Q118 by (p605 - p606);
  shift Q119 by (p605 - p606);

  p606 := get_point 1 Q119;
  p607 := get_point 2 Q119;
  p608 := (xpart p605, ypart p607);
  p609 := mediate(p605, p607);

  Q119 := p605 -- p606 -- p607 -- p608 -- cycle;

  draw Q119 with_color gray;
  draw Q118;

  p610 := p605 shifted (-offset, -offset);
  p611 := p606 shifted (offset, -offset);
  p612 := p607 shifted (offset, offset);
  p613 := p608 shifted (-offset, offset);

  Q120 := p610 -- p611 -- p612 -- p613 -- cycle;

  draw Q120 with_color red;
  
  
  if do_labels:
    dotlabel.bot("$p_{605}$", p605);
    dotlabel.bot("$p_{606}$", p606);
    dotlabel.top("$p_{607}$", p607);
    dotlabel.top("$p_{608}$", p608);
    dotlabel.rt("$p_{609}$", p609);
    dotlabel.bot("$p_{610}$", p610);
    dotlabel.bot("$p_{611}$", p611);
    dotlabel.top("$p_{612}$", p612);
    dotlabel.top("$p_{613}$", p613);
    label.lrt("$Q_{118}$", get_point 0 Q118 shifted (2mm, -2mm));
    label.ulft("$Q_{119}$", p606) with_color dark_gray;
    label.top("$Q_{120}$", mediate(p612, p613)) with_color red;
    label.bot("$c_{har0~(Gamma)}$", mediate(p610, p611) shifted (0, -.5));
  fi;
  
  if false: % true:
    n := (size Q118) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q118 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q118) with_color red;
    endfor;
  fi;
  
%% ** (2) Lambda, Greek letter, uppercase

  glyph Lambda;
  Lambda := get_glyph 3 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Lambda;

  scale qv (10, 10);
  
  Q121 := qv0;
  Q122 := qv1;

  p614 := p606 shifted (8, 0);
  p615 := (xpart get_point 5 Q121, ypart get_point 6 Q121);

  shift Q121 by (p614 - p615);
  shift Q122 by (p614 - p615);
  
  p615 := get_point 1 Q122;
  p616 := get_point 2 Q122;
  p617 := (xpart p614, ypart p616);
  p618 := mediate(p614, p616);

  Q122 := p614 -- p615 -- p616 -- p617 -- cycle;

  draw Q122 with_color gray;
  draw Q121;

  p619 := p614 shifted (-offset, -offset);
  p620 := p615 shifted (offset, -offset);
  p621 := p616 shifted (offset, offset);
  p622 := p617 shifted (-offset, offset);

  
  Q123 := p619 -- p620 -- p621 -- p622 -- cycle;

  draw Q123 with_color red;

  if do_labels:
    dotlabel.bot("$p_{614}$", p614);
    dotlabel.bot("$p_{615}$", p615);
    dotlabel.top("$p_{616}$", p616);
    dotlabel.top("$p_{617}$", p617);
    dotlabel.bot("$p_{618}$", p618);
    dotlabel.bot("$p_{619}$", p619);
    dotlabel.bot("$p_{620}$", p620);
    dotlabel.top("$p_{621}$", p621);
    dotlabel.top("$p_{622}$", p622);
    label.lrt("$Q_{121}$", p617 shifted (-4mm, -4mm));
    label.rt("$Q_{122}$", mediate(p615, p616)) with_color dark_gray;
    label.top("$Q_{123}$", mediate(p621, p622)) with_color red;
    label.bot("$c_{har3~(Lambda)}$", mediate(p619, p620) shifted (0, -.5));
  fi;
  
  if false: % true:
    n := (size Q121) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q121 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q121) with_color red;
    endfor;
  fi;

  
%% ** (2)

    
endfig with_projection parallel_x_y;
fig_ctr += 1;


%% * (1)

beginfig(fig_ctr);

%% ** (2)
  
  draw frame with_pen big_square_pen;
  output current_picture with_projection parallel_x_y;
  clear current_picture;

%% ** (2) Pi (Greek letter, uppercase)

  glyph Pi;
  Pi := get_glyph 5 from "cmssbx10";

  clear qv;
  
  qv := get_paths from Pi;

  scale qv (10, 10);
  
  Q124 := qv0;
  Q125 := qv1;

  p623 := (xpart get_point 4 Q124, ypart get_point 5 Q124);

  shift Q124 by (p485 - p623);
  shift Q125 by (p485 - p623);

  p623 := get_point 1 Q125;
  p624 := get_point 2 Q125;
  p625 := (xpart p485, ypart p624);
  p626 := mediate(p485, p624);

  Q125 := p485 -- p623 -- p624 -- p625 -- cycle;
  
  draw Q125 with_color gray;
  draw Q124;

  p627 := p485 shifted (-offset, -offset);
  p628 := p623 shifted (offset, -offset);
  p629 := p624 shifted (offset, offset);
  p630 := p625 shifted (-offset, offset);

  Q126 := p627 -- p628 -- p629 -- p630 -- cycle;

  draw Q126 with_color red;
  

  if do_labels:
    dotlabel.bot("$p_{485}$", p485);
    dotlabel.bot("$p_{623}$", p623);
    dotlabel.top("$p_{624}$", p624);
    dotlabel.top("$p_{625}$", p625);
    dotlabel.bot("$p_{626}$", p626);
    dotlabel.bot("$p_{627}$", p627);
    dotlabel.bot("$p_{628}$", p628);
    dotlabel.top("$p_{629}$", p629);
    dotlabel.top("$p_{630}$", p630);
    label.bot("$Q_{124}$", mediate(get_point 8 Q124, get_point 9 Q124));
    label.top("$Q_{125}$", mediate(p485, p623)) with_color dark_gray;
    label.top("$Q_{126}$", mediate(p629, p630)) with_color red;
    label.bot("$c_{har5~(Pi)}$", mediate(p627, p628) shifted (0, -.5));
  fi;
  
  if false: % true:
    n := (size Q124) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q124 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q124) with_color red;
    endfor;
  fi;

%% ** (2) a_lc ('a', lowercase)

  glyph a_lc;
  a_lc := get_glyph 97 from "cmssbx10";

  clear qv;
  
  qv := get_paths from a_lc;

  scale qv (10, 10);
  
  Q127 := qv0;
  Q128 := qv1;
  Q129 := qv2;	

  p631 := p623 shifted (8, 0);



  p632 := (xpart get_point 10 Q127, ypart get_point 11 Q127);

  shift Q127 by (p631 - p632);
  shift Q128 by (p631 - p632);
  shift Q129 by (p631 - p632);

  p632 := get_point 1 Q129;
  p633 := get_point 2 Q129;
  p634 := (xpart p631, ypart p633);
  p635 := mediate(p631, p633);

  Q129 := p631 -- p632 -- p633 -- p634 -- cycle;
  
  draw Q129 with_color gray;
  draw Q127;
  draw Q128;


  p636 := p631 shifted (-offset, -offset);
  p637 := p632 shifted (offset, -offset);
  p638 := p633 shifted (offset, offset);
  p639 := p634 shifted (-offset, offset);

  Q130 := p636 -- p637 -- p638 -- p639 -- cycle;

  draw Q130 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{631}$", p631);
    dotlabel.bot("$p_{632}$", p632);
    dotlabel.top("$p_{633}$", p633);
    dotlabel.top("$p_{634}$", p634);
    dotlabel.ulft("$p_{635}$", p635);
    dotlabel.bot("$p_{636}$", p636);
    dotlabel.bot("$p_{637}$", p637);
    dotlabel.top("$p_{638}$", p638);
    dotlabel.top("$p_{639}$", p639);
    label.rt("$Q_{127}$", mediate(p632, p633));
    label("$Q_{128}$", p635 shifted (0, -1));
    label.lft("$Q_{129}$", mediate(p634, p631, .25)) with_color dark_gray;
    label.top("$Q_{130}$", mediate(p638, p639)) with_color red;
  fi;
  
  if false: % true:
    n := (size Q127) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q127 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q127) with_color red;
    endfor;
  fi;

%% ** (2) u_lc ('u', lowercase)

  glyph u_lc;
  u_lc := get_glyph 117 from "cmssbx10";

  clear qv;
  
  qv := get_paths from u_lc;

  scale qv (10, 10);

  Q131 := qv0;
  Q132 := qv1;

  p640 := p632 shifted (8, 0);
  p641 := (xpart get_point 11 Q131, ypart get_point 12 Q131);

  shift Q131 by (p640 - p641);
  shift Q132 by (p640 - p641);

  p641 := get_point 1 Q132;
  p642 := get_point 2 Q132;
  p643 := (xpart p640, ypart p642);
  p644 := mediate(p640, p642);

  Q132 := p640 -- p641 -- p642 -- p643 -- cycle;
  
  draw Q132 with_color gray;
  draw Q131;

  p645 := p640 shifted (-offset, -offset);
  p646 := p641 shifted (offset, -offset);
  p647 := p642 shifted (offset, offset);
  p648 := p643 shifted (-offset, offset);

  Q133 := p645 -- p646 -- p647 -- p648 -- cycle;

  draw Q133 with_color red;
  
  if do_labels:
    dotlabel.bot("$p_{640}$", p640);
    dotlabel.bot("$p_{641}$", p641);
    dotlabel.top("$p_{642}$", p642);
    dotlabel.top("$p_{643}$", p643);
    dotlabel.bot("$p_{644}$", p644);
    dotlabel.bot("$p_{645}$", p645);
    dotlabel.bot("$p_{646}$", p646);
    dotlabel.top("$p_{647}$", p647);
    dotlabel.top("$p_{648}$", p648);
    label.lft("$Q_{131}$", mediate(p640, p643));
    label.top("$Q_{132}$", mediate(p643, p642)) with_color dark_gray;
    label.top("$Q_{133}$", mediate(p647, p648)) with_color red;
  fi;

  if false: % true:
    n := (size Q131) - 1;
    for i = 0 upto n:
      s := "$" & decimal i & "$";
      drawdot get_point (i) Q131 with_color red with_pen pencircle scaled (1mm, 1mm, 1mm);
      dotlabel.bot(s, get_point (i) Q131) with_color red;
    endfor;
  fi;
  

%% ** (2)  
  
    
endfig with_projection parallel_x_y;
fig_ctr += 1;




endinput;

%% * (1) Emacs-Lisp code for use in indirect buffers when using the          
%%       GNU Emacs editor.  The local variable list is not fvaluated when an 
%%       indirect buffer is visited, so it's necessary to evaluate the       
%%       following s-expression in order to use the facilities normally      
%%       accessed via the local variables list.                              
%%       LDF 2004.02.12.

%% (progn (metafont-mode) (outline-minor-mode t) (setq fill-column 80) (ignore '(  
%% )) (setq outline-regexp "%% [*\f]+"))

%% * (1) Local variables for Emacs.

%% Local Variables:
%% mode:Metapost
%% eval:(outline-minor-mode t)
%% eval:(read-abbrev-file abbrev-file-name)
%% outline-regexp:"%% [*\f]+"
%% End:
