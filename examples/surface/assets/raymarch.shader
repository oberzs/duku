vert.spv                                                                                                                    00000005570 00000000000 0005350                                                                                                      ustar                                                                                                                                                                                                                                                          #     6                 GLSL.std.450                      main             !   1   3   4   5        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         vertex(      gl_PerVertex             gl_Position         gl_PointSize            gl_ClipDistance         gl_CullDistance               in_position      out_uv    !   in_uv     %   Light     %       coords    %      color     (   WorldObject   (       cam_mat   (      light_mat     (      lights    (      cam_pos   (      time      (      shadow_index      *   world     +   MaterialObject    +       albedo_tint   +      font_width    +      font_border_tint      +      font_edge     +      font_border_offset    +      font_border_width     +      font_border_edge      +      arg_1     +      arg_2     +   	   arg_3     +   
   arg_4     -   material      .   Constants     .       model_mat     .      albedo_index      0   object    1   in_normal     3   out_position      4   out_normal    5   out_ls_position H                H              H              H              G        G            G           G  !         H  %       #       H  %      #      G  '          H  (          H  (       #       H  (             H  (         H  (      #   @   H  (            H  (      #   �   H  (      #      H  (      #     H  (      #     G  (      G  *   "       G  *   !       H  +       #       H  +      #      H  +      #      H  +      #      H  +      #       H  +      #   (   H  +      #   ,   H  +      #   0   H  +      #   @   H  +   	   #   P   H  +   
   #   `   G  +      G  -   "      G  -   !       H  .          H  .       #       H  .             H  .      #   @   G  .      G  1         G  3          G  4         G  5              !                   	           
           +  
                         	                        ;                       +                                   ;           +          �?         	                          ;                        ;      !        $   	        %   	   	   +  
   &        '   %   &     (   $   $   '               )      (   ;  )   *        +                        	   	   	   	      ,      +   ;  ,   -        .   $         /   	   .   ;  /   0   	   ;     1         2         ;  2   3      ;  2   4      ;     5      6               �     9     #      �  8  6               �     =           Q               Q              Q              P  	                  A              >        =     "   !   >     "   �  8                                                                                                                                          frag.spv                                                                                                                    00000023054 00000000000 0005304                                                                                                      ustar                                                                                                                                                                                                                                                          #     e                GLSL.std.450              
       main      I  a  b  d               �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         rotate(f1;       a        sphere(vf3;vf3;f1;       pos      c        r        box(vf3;vf3;         pos      size         get_dist_to_scene(vf3;       pos   !   ray_march(vf3;vf3;       ray_orig          ray_dir   %   get_normal(vf3;   $   pos   (   get_light(vf3;    '   pos   *   fragment(     ,   s     /   c     >   dist      L   dist      b   plane_dist    e   box_pos   m   Light     m       coords    m      color     q   WorldObject   q       cam_mat   q      light_mat     q      lights    q      cam_pos   q      time      q      shadow_index      s   world     u   param        box_dist      �   param     �   param     �   sphere_dist   �   param     �   param     �   param     �   morph_dist    �   dist      �   dist_orig     �   i     �   pos   �   dist_scene    �   param     �   dist      �   param     �   e     �   normal    �   param     �   param     �   param     �   light_pos     �   light_dir     �   normal    �   param     �   light     �   dist        param       param       uv      in_uv     (  ray_orig      +  ray_dir   4  color     6  dist      7  param     9  param     <  pos   B  light     C  param     I  out_color     P  MaterialObject    P      albedo_tint   P     font_width    P     font_border_tint      P     font_edge     P     font_border_offset    P     font_border_width     P     font_border_edge      P     arg_1     P     arg_2     P  	   arg_3     P  
   arg_4     R  material      S  Constants     S      model_mat     S     albedo_index      U  object    Z  textures      _  samplers      a  in_position   b  in_normal     d  in_ls_position  H  m       #       H  m      #      G  o          H  q          H  q       #       H  q             H  q         H  q      #   @   H  q            H  q      #   �   H  q      #      H  q      #     H  q      #     G  q      G  s   "       G  s   !       G          G  I         H  P      #       H  P     #      H  P     #      H  P     #      H  P     #       H  P     #   (   H  P     #   ,   H  P     #   0   H  P     #   @   H  P  	   #   P   H  P  
   #   `   G  P     G  R  "      G  R  !       H  S         H  S      #       H  S            H  S     #   @   G  S     G  Z  "      G  Z  !       G  _  "      G  _  !      G  a         G  b        G  d             !                                          	         !  
   	                             !                 !              !           !  #         +     7     �?+     8         Q           +  Q   R       +  Q   U      +  Q   X      +     g     �@,     h   8   7   g     k           l   k        m   k   k   +  Q   n        o   m   n     p            q   l   l   o         p      r      q   ;  r   s      +  p   t         v         ,     �   7   7   7   +     �   �̌?+     �      ?   �      p   +  p   �       +  p   �   d     �   +     �     �B+     �   o�:+  p   �         �         ,     �   �   8   +     �     �@+     �      A,     �   �   �   �   +          @+     
  
�#<           ;         +     #    ��+     )    �@,     *  8   )  8   +     0  ���>,     5  8   8   8      H     k   ;  H  I       P                       k   k   k   k      Q     P  ;  Q  R       S  l   p      T  	   S  ;  T  U  	    	 V                           +  Q   W  d     X  V  W     Y      X  ;  Y  Z        [  +  Q   \       ]  [  \     ^      ]  ;  ^  _         `        ;  `  a     ;  `  b        c     k   ;  c  d     6               �     9     O  *   �  8  6  	          
   7        �     ;     ,      ;     /      =     -           .         -   >  ,   .   =     0           1         0   >  /   1   =     2   /   =     3   ,        4   3   =     5   ,   =     6   /   P     9   2   4   P     :   5   6   P  	   ;   9   :   �  ;   8  6               7        7        7        �     ;     >      =     ?      =     @      �     A   ?   @        B      B   A   =     C      �     D   B   C   >  >   D   =     E   >   �  E   8  6               7        7        �     ;     L      =     H           I         H   =     J      �     K   I   J   >     K   =     M      P     N   8   8   8        O      (   M   N        P      B   O   A     S      R   =     T   S   A     V      U   =     W   V   A     Y      X   =     Z   Y        [      (   W   Z        \      (   T   [        ]      %   \   8   �     ^   P   ]   >  L   ^   =     _   L   �  _   8  6               7        �     ;     b      ;     e      ;     u      ;           ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      A     c      U   =     d   c   >  b   d   =     f      >  e   f   =     i   e   �     j   i   h   >  e   j   A  v   w   s   t   =     x   w   >  u   x   9  	   y      u   =     z   e   O     {   z   z          �     |   {   y   =     }   e   O     ~   }   |            >  e   ~   =     �   e   >  �   �   >  �   �   9     �      �   �   >     �   =     �      >  �   �   >  �   h   >  �   �   9     �      �   �   �   >  �   �   =     �      =     �   �   A  v   �   s   t   =     �   �        �         �   �     �   �   �   �     �   �   �        �      .   �   �   �   >  �   �   =     �   �   =     �   b        �      %   �   �   >  �   �   =     �   �   �  �   8  6     !          7        7         �  "   ;     �      ;  �   �      ;     �      ;     �      ;     �      >  �   8   >  �   �   �  �   �  �   �  �   �       �  �   �  �   =  p   �   �   �  �   �   �   �   �  �   �   �   �  �   =     �      =     �       =     �   �   �     �   �   �   �     �   �   �   >  �   �   =     �   �   >  �   �   9     �      �   >  �   �   =     �   �   =     �   �   �     �   �   �   >  �   �   =     �   �   �  �   �   �   �   =     �   �   �  �   �   �   �   �  �   �   �   �   �  �       �  �   �   �   �  �   �  �   �  �   �  �   �  �   =  p   �   �   �  p   �   �   �   >  �   �   �  �   �  �   =     �   �   �  �   8  6     %       #   7     $   �  &   ;     �      ;     �      ;  �   �      ;     �      ;     �      ;     �      ;     �      =     �   $   >  �   �   9     �      �   >  �   �   >  �   �   =     �   �   =     �   $   =     �   �   O     �   �   �             �     �   �   �   >  �   �   9     �      �   =     �   $   =     �   �   O     �   �   �             �     �   �   �   >  �   �   9     �      �   =     �   $   =     �   �   O     �   �   �             �     �   �   �   >  �   �   9     �      �   P     �   �   �   �   P     �   �   �   �   �     �   �   �   >  �   �   =     �   �        �      E   �   �  �   8  6     (          7     '   �  )   ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;          ;          >  �   �   =     �   �   =     �   '   �     �   �   �        �      E   �   >  �   �   =     �   '   >  �   �   9     �   %   �   >  �   �   =     �   �   =     �   �   �     �   �   �   �     �   �   �   �     �   �   �        �      +   �   8   7   >  �   �   =     �   '   =     �   �   �        �   �   �            �       �     >      =       �   >      9       !       >  �     A       '   U   =     	    �  �     	  
  �        �        �    =       �   =       �   =       '   �                     B     �  �         �    �    �  �       )       �        �        �    =       �   �         �   >  �     �    �    =       �   �    8  6     *          �  +   ;  �        ;     (     ;     +     ;     4     ;     6     ;     7     ;     9     ;     <     ;     B     ;     C     =         �            P     !  7   7   �     "     !  >    "  A     $    U   =     %  $  �     &  %  #  A     '    U   >  '  &  >  (  *  A     ,    R   =     -  ,  A     .    U   =     /  .  �     1  /  0  P     2  -  1  7        3     E   2  >  +  3  >  4  5  =     8  (  >  7  8  =     :  +  >  9  :  9     ;  !   7  9  >  6  ;  =     =  (  =     >  +  =     ?  6  �     @  >  ?  �     A  =  @  >  <  A  =     D  <  >  C  D  9     E  (   C  >  B  E  =     F  B  P     G  F  F  F  >  4  G  =     J  4  Q     K  J      Q     L  J     Q     M  J     P  k   N  K  L  M  7   >  I  N  �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      